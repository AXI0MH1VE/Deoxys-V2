"""
AxiomHive Contract Summarization Pipeline v4.1
Deterministic Legal Summarizer - Zero Entropy Law (C=0)
Created by Alexis M. Adams
"""

import json
import re
from typing import Dict, List, Optional, Any
from datetime import datetime
import hashlib

class ContractPipeline:
    """
    Deterministic contract summarization pipeline.
    Enforces strict cardinality limits and schema validation.
    """
    
    def __init__(self, frozen_seed: str = "PLACEHOLDER_UUID"):
        self.frozen_seed = frozen_seed
        self.max_obligations = 10
        self.max_risk_flags = 20
        
    def ingest_contract(self, source_blob: str) -> Dict[str, Any]:
        """
        Node: input_ingest
        Ingests raw contract text from upstream source.
        """
        return {
            "contract_text": source_blob,
            "ingest_timestamp": datetime.utcnow().isoformat() + "Z"
        }
    
    def extract_metadata(self, contract_text: str) -> Dict[str, Any]:
        """
        Node: extract_metadata
        Extract parties and dates with ISO-8601 format enforcement.
        """
        # Deterministic extraction patterns
        parties = self._extract_parties(contract_text)
        
        # Extract dates (ISO-8601 format)
        dates = self._extract_dates(contract_text)
        
        # Extract jurisdiction
        jurisdiction = self._extract_jurisdiction(contract_text)
        
        return {
            "parties": parties,
            "effective_date": dates.get("effective", ""),
            "termination_date": dates.get("termination", ""),
            "jurisdiction": jurisdiction
        }
    
    def extract_obligations(self, contract_text: str, parties: List[str]) -> Dict[str, Any]:
        """
        Node: extract_obligations
        Extract obligations with strict cardinality enforcement (max 10).
        """
        obligations = []
        
        # Pattern-based extraction (deterministic)
        obligation_patterns = [
            r"(?:shall|must|will|agrees to|obligated to)\s+([^\.]+)",
            r"(?:obligation|duty|requirement)[\s:]+([^\.]+)",
            r"(?:party|parties)[\s]+(?:shall|must|will)[\s]+([^\.]+)"
        ]
        
        for pattern in obligation_patterns:
            matches = re.finditer(pattern, contract_text, re.IGNORECASE)
            for match in matches:
                if len(obligations) >= self.max_obligations:
                    break
                    
                description = match.group(1).strip()
                # Assign to first party by default (deterministic)
                party = parties[0] if parties else "Unknown"
                
                obligations.append({
                    "party": party,
                    "description": description[:200],  # Limit length
                    "due_date": "",  # Extract separately if needed
                    "category": self._categorize_obligation(description)
                })
                
            if len(obligations) >= self.max_obligations:
                break
        
        return {
            "key_obligations": obligations[:self.max_obligations]
        }
    
    def detect_risks(self, key_obligations: List[Dict], dates: Dict[str, str]) -> Dict[str, Any]:
        """
        Node: detect_risks
        Derive risk flags with strict cardinality enforcement (max 20).
        """
        risk_flags = []
        
        # Risk detection patterns (deterministic)
        high_risk_keywords = ["penalty", "breach", "termination", "liability", "damages"]
        medium_risk_keywords = ["deadline", "requirement", "compliance", "audit"]
        low_risk_keywords = ["notification", "report", "disclosure"]
        
        # Check obligations for risks
        for obligation in key_obligations:
            if len(risk_flags) >= self.max_risk_flags:
                break
                
            desc = obligation.get("description", "").lower()
            
            # Determine severity
            severity = "low"
            if any(kw in desc for kw in high_risk_keywords):
                severity = "high"
            elif any(kw in desc for kw in medium_risk_keywords):
                severity = "medium"
            
            risk_flags.append({
                "severity": severity,
                "category": obligation.get("category", "general"),
                "description": f"Risk associated with: {obligation.get('description', '')[:100]}"
            })
        
        # Check date-related risks
        if dates.get("termination_date"):
            try:
                term_date = datetime.fromisoformat(dates["termination_date"].replace("Z", "+00:00"))
                days_until = (term_date - datetime.utcnow()).days
                if days_until < 90 and len(risk_flags) < self.max_risk_flags:
                    risk_flags.append({
                        "severity": "high",
                        "category": "temporal",
                        "description": f"Contract termination within {days_until} days"
                    })
            except:
                pass
        
        return {
            "risk_flags": risk_flags[:self.max_risk_flags]
        }
    
    def validate_structures(self, compiled_summary: Dict[str, Any]) -> Dict[str, Any]:
        """
        Node: validate_structures
        Validate against final spec and emit typed error codes.
        """
        failure_codes = []
        is_valid = True
        
        # Check required fields
        required_fields = ["parties", "key_obligations", "risk_flags"]
        for field in required_fields:
            if field not in compiled_summary:
                failure_codes.append("MISSING_REQUIRED_FIELD")
                is_valid = False
        
        # Check cardinality
        if "key_obligations" in compiled_summary:
            if len(compiled_summary["key_obligations"]) > self.max_obligations:
                failure_codes.append("CARDINALITY_EXCEEDED")
                is_valid = False
        
        if "risk_flags" in compiled_summary:
            if len(compiled_summary["risk_flags"]) > self.max_risk_flags:
                failure_codes.append("CARDINALITY_EXCEEDED")
                is_valid = False
        
        # Check schema compliance
        if "key_obligations" in compiled_summary:
            for obligation in compiled_summary["key_obligations"]:
                if "party" not in obligation or "description" not in obligation:
                    failure_codes.append("SCHEMA_VIOLATION")
                    is_valid = False
        
        if "risk_flags" in compiled_summary:
            for risk in compiled_summary["risk_flags"]:
                if "severity" not in risk or "description" not in risk:
                    failure_codes.append("SCHEMA_VIOLATION")
                    is_valid = False
        
        return {
            "is_valid": is_valid,
            "failure_codes": list(set(failure_codes))  # Remove duplicates
        }
    
    def route_on_validation(self, is_valid: bool, failure_codes: List[str]) -> Dict[str, Any]:
        """
        Node: route_on_validation
        Route to output or error sink based on validation result.
        """
        if is_valid:
            return {
                "route_decision": "emit_output",
                "final_error_payload": None
            }
        else:
            return {
                "route_decision": "emit_error",
                "final_error_payload": {
                    "error_codes": failure_codes,
                    "message": f"Validation failed: {', '.join(failure_codes)}"
                }
            }
    
    def process_contract(self, contract_text: str) -> Dict[str, Any]:
        """
        Execute the complete pipeline.
        """
        # Node 1: input_ingest
        ingested = self.ingest_contract(contract_text)
        contract_text = ingested["contract_text"]
        
        # Node 2: extract_metadata
        metadata = self.extract_metadata(contract_text)
        
        # Node 3: extract_obligations
        obligations = self.extract_obligations(contract_text, metadata["parties"])
        
        # Node 4: detect_risks
        dates = {
            "effective": metadata.get("effective_date", ""),
            "termination": metadata.get("termination_date", "")
        }
        risks = self.detect_risks(obligations["key_obligations"], dates)
        
        # Compile summary
        compiled_summary = {
            "parties": metadata["parties"],
            "key_obligations": obligations["key_obligations"],
            "risk_flags": risks["risk_flags"],
            "effective_date": metadata.get("effective_date", ""),
            "termination_date": metadata.get("termination_date", ""),
            "jurisdiction": metadata.get("jurisdiction", "")
        }
        
        # Node 5: validate_structures
        validation = self.validate_structures(compiled_summary)
        
        # Node 6: route_on_validation
        routing = self.route_on_validation(validation["is_valid"], validation["failure_codes"])
        
        if routing["route_decision"] == "emit_output":
            return {
                "status": "success",
                "summary": compiled_summary,
                "validation": validation
            }
        else:
            return {
                "status": "error",
                "error": routing["final_error_payload"],
                "partial_summary": compiled_summary
            }
    
    def _extract_parties(self, text: str) -> List[str]:
        """Extract party names deterministically."""
        parties = []
        # Pattern: "Party A", "Company X", "between [Name] and [Name]"
        patterns = [
            r"between\s+([A-Z][^,\s]+(?:\s+[A-Z][^,\s]+)*)\s+and\s+([A-Z][^,\s]+(?:\s+[A-Z][^,\s]+)*)",
            r"Party\s+([A-Z])\s*[:\(]",
            r"([A-Z][a-zA-Z\s]+(?:Inc\.|LLC|Corp|Ltd))"
        ]
        
        for pattern in patterns:
            matches = re.finditer(pattern, text)
            for match in matches:
                if match.groups():
                    for group in match.groups():
                        if group and group.strip():
                            party = group.strip()
                            if party not in parties:
                                parties.append(party)
        
        return parties[:10]  # Limit parties
    
    def _extract_dates(self, text: str) -> Dict[str, str]:
        """Extract dates in ISO-8601 format."""
        dates = {}
        
        # ISO-8601 pattern
        iso_pattern = r"(\d{4}-\d{2}-\d{2}(?:T\d{2}:\d{2}:\d{2}(?:Z|[+-]\d{2}:\d{2})?)?)"
        iso_matches = list(re.finditer(iso_pattern, text))
        
        # Common date patterns
        date_patterns = [
            r"(?:effective|commencement)[\s:]+(?:date|on)?[\s:]+(\d{1,2}[/-]\d{1,2}[/-]\d{2,4})",
            r"(?:termination|expiration|end)[\s:]+(?:date|on)?[\s:]+(\d{1,2}[/-]\d{1,2}[/-]\d{2,4})"
        ]
        
        for pattern in date_patterns:
            matches = re.finditer(pattern, text, re.IGNORECASE)
            for match in matches:
                date_str = match.group(1)
                # Convert to ISO-8601 (simplified)
                try:
                    # Try parsing common formats
                    if "/" in date_str:
                        parts = date_str.split("/")
                        if len(parts) == 3:
                            if len(parts[2]) == 2:
                                parts[2] = "20" + parts[2]
                            iso_date = f"{parts[2]}-{parts[0].zfill(2)}-{parts[1].zfill(2)}"
                            if "effective" in match.group(0).lower():
                                dates["effective"] = iso_date
                            elif "termination" in match.group(0).lower() or "expiration" in match.group(0).lower():
                                dates["termination"] = iso_date
                except:
                    pass
        
        # Use ISO matches if found
        if iso_matches:
            dates["effective"] = iso_matches[0].group(1)
            if len(iso_matches) > 1:
                dates["termination"] = iso_matches[1].group(1)
        
        return dates
    
    def _extract_jurisdiction(self, text: str) -> str:
        """Extract jurisdiction deterministically."""
        jurisdiction_patterns = [
            r"jurisdiction[\s:]+(?:of|is|in)?[\s:]+([A-Z][^\.\n,]+)",
            r"governed by[\s]+(?:the\s+)?laws?[\s]+of[\s]+([A-Z][^\.\n,]+)",
            r"([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)\s+law"
        ]
        
        for pattern in jurisdiction_patterns:
            match = re.search(pattern, text, re.IGNORECASE)
            if match:
                return match.group(1).strip()
        
        return "Unknown"
    
    def _categorize_obligation(self, description: str) -> str:
        """Categorize obligation deterministically."""
        desc_lower = description.lower()
        
        if any(kw in desc_lower for kw in ["payment", "fee", "cost", "price"]):
            return "financial"
        elif any(kw in desc_lower for kw in ["deliver", "provide", "supply"]):
            return "delivery"
        elif any(kw in desc_lower for kw in ["maintain", "support", "service"]):
            return "maintenance"
        elif any(kw in desc_lower for kw in ["report", "notify", "inform"]):
            return "reporting"
        else:
            return "general"


def main():
    """CLI interface for contract pipeline."""
    import sys
    import argparse
    
    parser = argparse.ArgumentParser(description='AxiomHive Contract Pipeline')
    parser.add_argument('--contract', type=str, required=True, help='Contract text')
    parser.add_argument('--frozen-seed', type=str, default='PLACEHOLDER_UUID', help='Frozen seed')
    
    args = parser.parse_args()
    
    pipeline = ContractPipeline(frozen_seed=args.frozen_seed)
    result = pipeline.process_contract(args.contract)
    
    print(json.dumps(result, indent=2))
    
    return 0 if result["status"] == "success" else 1


if __name__ == "__main__":
    sys.exit(main())

