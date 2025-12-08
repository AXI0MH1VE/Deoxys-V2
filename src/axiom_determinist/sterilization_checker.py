#!/usr/bin/env python3
"""
AxiomDeterminist Sterilization Checker
Detects placeholders, stubs, and lazy code patterns
Created by Alexis M. Adams
"""

import re
import ast
import json
import sys
from typing import List, Dict, Any, Optional
from pathlib import Path

# Banned patterns for sterilization
BANNED_PATTERNS = [
    (r'\bTODO\b', 'FATAL'),
    (r'\bFIXME\b', 'FATAL'),
    (r'\bXXX\b', 'FATAL'),
    (r'\bHACK\b', 'FATAL'),
    (r'NotImplementedError', 'FATAL'),
    (r'NotImplemented', 'FATAL'),
    (r'omitted for brevity', 'FATAL'),
    (r'rest of code', 'FATAL'),
    (r'left as an exercise', 'FATAL'),
    (r'implementation omitted', 'FATAL'),
    (r'unimplemented!\(\)', 'FATAL'),  # Rust
    (r'todo!\(\)', 'FATAL'),  # Rust
]

class SterilizationError:
    def __init__(self, severity: str, message: str, file: Optional[str] = None, 
                 line: Optional[int] = None, column: Optional[int] = None):
        self.severity = severity
        self.message = message
        self.file = file
        self.line = line
        self.column = column
    
    def to_dict(self) -> Dict[str, Any]:
        return {
            "severity": self.severity,
            "message": self.message,
            "file": self.file,
            "line": self.line,
            "column": self.column,
            "error_type": "SterilizationViolation"
        }

def check_text_patterns(code: str, file_path: Optional[str] = None) -> List[SterilizationError]:
    """Check for banned text patterns"""
    errors = []
    
    for line_num, line in enumerate(code.split('\n'), 1):
        for pattern, severity in BANNED_PATTERNS:
            if re.search(pattern, line, re.IGNORECASE):
                errors.append(SterilizationError(
                    severity=severity,
                    message=f"Found banned pattern: {pattern}",
                    file=file_path,
                    line=line_num,
                ))
    
    return errors

def check_python_ast(code: str, file_path: Optional[str] = None) -> List[SterilizationError]:
    """Check Python AST for empty functions and stubs"""
    errors = []
    
    try:
        tree = ast.parse(code)
        
        for node in ast.walk(tree):
            # Check for function definitions with only 'pass'
            if isinstance(node, ast.FunctionDef):
                if len(node.body) == 1:
                    if isinstance(node.body[0], ast.Pass):
                        errors.append(SterilizationError(
                            severity='FATAL',
                            message=f"Function '{node.name}' contains only 'pass' statement",
                            file=file_path,
                            line=node.lineno,
                        ))
                    elif isinstance(node.body[0], ast.Expr) and isinstance(node.body[0].value, ast.Constant):
                        # Check for docstring-only functions
                        if len(node.body) == 1:
                            errors.append(SterilizationError(
                                severity='FATAL',
                                message=f"Function '{node.name}' contains only docstring",
                                file=file_path,
                                line=node.lineno,
                            ))
                
                # Check for NotImplementedError
                for stmt in node.body:
                    if isinstance(stmt, ast.Raise):
                        if isinstance(stmt.exc, ast.Call):
                            if isinstance(stmt.exc.func, ast.Name):
                                if stmt.exc.func.id == 'NotImplementedError':
                                    errors.append(SterilizationError(
                                        severity='FATAL',
                                        message=f"Function '{node.name}' raises NotImplementedError",
                                        file=file_path,
                                        line=stmt.lineno,
                                    ))
    
    except SyntaxError as e:
        errors.append(SterilizationError(
            severity='ERROR',
            message=f"Syntax error: {e.msg}",
            file=file_path,
            line=e.lineno,
            column=e.offset,
        ))
    
    return errors

def check_rust_patterns(code: str, file_path: Optional[str] = None) -> List[SterilizationError]:
    """Check Rust code for banned patterns"""
    errors = []
    
    for line_num, line in enumerate(code.split('\n'), 1):
        if 'unimplemented!()' in line or 'todo!()' in line:
            errors.append(SterilizationError(
                severity='FATAL',
                message="Found unimplemented!() or todo!() macro",
                file=file_path,
                line=line_num,
            ))
    
    return errors

def check_javascript_patterns(code: str, file_path: Optional[str] = None) -> List[SterilizationError]:
    """Check JavaScript/TypeScript code for banned patterns"""
    errors = []
    
    for line_num, line in enumerate(code.split('\n'), 1):
        if re.search(r'//\s*(TODO|FIXME|XXX|HACK)', line, re.IGNORECASE):
            errors.append(SterilizationError(
                severity='FATAL',
                message="Found TODO/FIXME comment",
                file=file_path,
                line=line_num,
            ))
        
        if 'throw new Error("TODO")' in line or 'throw new Error("NotImplemented")' in line:
            errors.append(SterilizationError(
                severity='FATAL',
                message="Found TODO/NotImplemented error throw",
                file=file_path,
                line=line_num,
            ))
    
    return errors

def check_code(code: str, language: str, file_path: Optional[str] = None) -> Dict[str, Any]:
    """Main sterilization check function"""
    errors = []
    
    # Check text patterns (language-agnostic)
    errors.extend(check_text_patterns(code, file_path))
    
    # Language-specific checks
    if language == 'python':
        errors.extend(check_python_ast(code, file_path))
    elif language == 'rust':
        errors.extend(check_rust_patterns(code, file_path))
    elif language in ['javascript', 'typescript']:
        errors.extend(check_javascript_patterns(code, file_path))
    
    # Convert to dict format
    error_dicts = [e.to_dict() for e in errors]
    
    # Determine if passed (no FATAL or ERROR severity)
    passed = not any(
        e['severity'] in ['FATAL', 'ERROR'] 
        for e in error_dicts
    )
    
    return {
        "passed": passed,
        "errors": error_dicts,
        "warnings": [],
    }

def main():
    if len(sys.argv) < 3:
        print(json.dumps({
            "error": "Usage: sterilization_checker.py <language> <code_file>"
        }), file=sys.stderr)
        sys.exit(1)
    
    language = sys.argv[1]
    code_file = sys.argv[2]
    
    try:
        with open(code_file, 'r', encoding='utf-8') as f:
            code = f.read()
    except Exception as e:
        print(json.dumps({
            "error": f"Failed to read file: {e}"
        }), file=sys.stderr)
        sys.exit(1)
    
    result = check_code(code, language, code_file)
    print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()

