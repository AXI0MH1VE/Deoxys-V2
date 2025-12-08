// AxiomHive Sovereign Manifold v2.1.0 - Frontend Controller
// Zero Entropy Law (C=0) - Deterministic UI

import { invoke } from '@tauri-apps/api/core';

// Initialize UI
document.addEventListener('DOMContentLoaded', async () => {
    console.log('AxiomHive Sovereign Manifold UI Initialized');
    
    // Model Control
    const runModelBtn = document.getElementById('runModelBtn');
    const clearBtn = document.getElementById('clearBtn');
    const modelInput = document.getElementById('modelInput');
    const modelOutput = document.getElementById('modelOutput');
    
    // TOON Parser
    const parseToonBtn = document.getElementById('parseToonBtn');
    const toonInput = document.getElementById('toonInput');
    const toonOutput = document.getElementById('toonOutput');
    
    // Risk Calculator
    const verifyRiskBtn = document.getElementById('verifyRiskBtn');
    const riskOutput = document.getElementById('riskOutput');
    
    // FHE Encryption
    const encryptBtn = document.getElementById('encryptBtn');
    const decryptBtn = document.getElementById('decryptBtn');
    const fheInput = document.getElementById('fheInput');
    const fheOutput = document.getElementById('fheOutput');
    
    let encryptedData = null;
    let fheKeys = null;

    // Run Mamba-2 Model
    runModelBtn.addEventListener('click', async () => {
        const input = modelInput.value.trim();
        if (!input) {
            modelOutput.innerHTML = '<p class="text-red-400">Error: Please enter a prompt</p>';
            return;
        }

        const stateDim = parseInt(document.getElementById('stateDim').value) || 128;
        const inputDim = parseInt(document.getElementById('inputDim').value) || 128;
        const temperature = parseFloat(document.getElementById('temperature').value) || 0.0;
        const frozenSeed = document.getElementById('frozenSeed').checked;

        runModelBtn.disabled = true;
        runModelBtn.textContent = 'Processing...';
        modelOutput.innerHTML = '<p class="text-white/50">Running Mamba-2 model...</p>';

        try {
            const result = await invoke('run_mamba_model', {
                prompt: input,
                stateDim,
                inputDim,
                temperature,
                frozenSeed
            });

            modelOutput.innerHTML = `
                <div class="space-y-2">
                    <p class="text-green-400 font-semibold">Model Output:</p>
                    <pre class="text-white whitespace-pre-wrap">${result.output || result}</pre>
                    ${result.metrics ? `
                        <div class="mt-4 pt-4 border-t border-miami-red/20">
                            <p class="text-miami-red font-semibold mb-2">Stability Metrics:</p>
                            <pre class="text-white/70 text-xs">${JSON.stringify(result.metrics, null, 2)}</pre>
                        </div>
                    ` : ''}
                </div>
            `;

            // Update risk score if available
            if (result.riskScore !== undefined) {
                document.getElementById('riskScore').textContent = result.riskScore;
            }
        } catch (error) {
            modelOutput.innerHTML = `<p class="text-red-400">Error: ${error}</p>`;
        } finally {
            runModelBtn.disabled = false;
            runModelBtn.textContent = 'Run Model';
        }
    });

    // Clear Output
    clearBtn.addEventListener('click', () => {
        modelInput.value = '';
        modelOutput.innerHTML = '<p class="text-white/50">Output will appear here...</p>';
    });

    // Parse TOON Data
    parseToonBtn.addEventListener('click', async () => {
        const input = toonInput.value.trim();
        if (!input) {
            toonOutput.textContent = 'Error: Please enter TOON data';
            return;
        }

        parseToonBtn.disabled = true;
        toonOutput.textContent = 'Parsing...';

        try {
            const result = await invoke('parse_toon_data', { data: input });
            toonOutput.innerHTML = `
                <div class="text-green-400 mb-1">Parsed successfully:</div>
                <pre class="text-white/70 text-xs overflow-auto">${result}</pre>
            `;
        } catch (error) {
            toonOutput.innerHTML = `<div class="text-red-400">Error: ${error}</div>`;
        } finally {
            parseToonBtn.disabled = false;
        }
    });

    // Verify Risk
    verifyRiskBtn.addEventListener('click', async () => {
        verifyRiskBtn.disabled = true;
        verifyRiskBtn.textContent = 'Verifying...';
        riskOutput.textContent = 'Running risk verification...';

        try {
            const result = await invoke('calculate_risk', { 
                input: 'AxiomHive_Sovereign_Manifold_v2.1.0' 
            });
            
            riskOutput.innerHTML = `
                <div class="space-y-1">
                    <pre class="text-white/70 text-xs whitespace-pre-wrap">${result}</pre>
                </div>
            `;

            // Parse and update UI
            const riskMatch = result.match(/Risk Score: (\d+)/);
            const entropyMatch = result.match(/Entropy Count: (\d+)/);
            
            if (riskMatch) {
                document.getElementById('riskScore').textContent = riskMatch[1];
                document.getElementById('riskScore').className = 
                    riskMatch[1] === '0' ? 'text-green-400 font-bold' : 'text-red-400 font-bold';
            }
            if (entropyMatch) {
                document.getElementById('entropyCount').textContent = entropyMatch[1];
            }
        } catch (error) {
            riskOutput.innerHTML = `<div class="text-red-400">Error: ${error}</div>`;
        } finally {
            verifyRiskBtn.disabled = false;
            verifyRiskBtn.textContent = 'Verify System';
        }
    });

    // FHE Encrypt
    encryptBtn.addEventListener('click', async () => {
        const input = parseInt(fheInput.value);
        if (isNaN(input)) {
            fheOutput.textContent = 'Error: Please enter a valid number';
            return;
        }

        encryptBtn.disabled = true;
        fheOutput.textContent = 'Encrypting...';

        try {
            const result = await invoke('encrypt_fhe', { message: input });
            encryptedData = result.ciphertext;
            fheKeys = result.keys;
            fheOutput.innerHTML = `
                <div class="text-green-400 mb-1">Encrypted successfully</div>
                <div class="text-white/70 text-xs break-all">Ciphertext: ${result.ciphertext.substring(0, 100)}...</div>
            `;
        } catch (error) {
            fheOutput.innerHTML = `<div class="text-red-400">Error: ${error}</div>`;
        } finally {
            encryptBtn.disabled = false;
        }
    });

    // FHE Decrypt
    decryptBtn.addEventListener('click', async () => {
        if (!encryptedData || !fheKeys) {
            fheOutput.textContent = 'Error: Please encrypt data first';
            return;
        }

        decryptBtn.disabled = true;
        fheOutput.textContent = 'Decrypting...';

        try {
            const result = await invoke('decrypt_fhe', { 
                ciphertext: encryptedData,
                keys: fheKeys
            });
            fheOutput.innerHTML = `
                <div class="text-green-400 mb-1">Decrypted successfully</div>
                <div class="text-white font-semibold">Plaintext: ${result}</div>
            `;
        } catch (error) {
            fheOutput.innerHTML = `<div class="text-red-400">Error: ${error}</div>`;
        } finally {
            decryptBtn.disabled = false;
        }
    });

    // Initialize system status
    await updateSystemStatus();
});

async function updateSystemStatus() {
    try {
        const status = await invoke('get_system_status');
        // Update status indicators if needed
    } catch (error) {
        console.error('Failed to get system status:', error);
    }
}

