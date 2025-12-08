// AxiomHive Sovereign Manifold v2.1.0 - Elite Frontend Controller
// Zero Entropy Law (C=0) - Cutting-Edge UI Experience

import { invoke } from '@tauri-apps/api/core';

// Initialize Elite UI
document.addEventListener('DOMContentLoaded', async () => {
    console.log('🚀 AxiomHive Sovereign Manifold Elite UI Initialized');
    
    // Initialize system time
    updateSystemTime();
    setInterval(updateSystemTime, 1000);
    
    // Model Control
    const runModelBtn = document.getElementById('runModelBtn');
    const clearBtn = document.getElementById('clearBtn');
    const resetModelBtn = document.getElementById('resetModelBtn');
    const modelInput = document.getElementById('modelInput');
    const modelOutput = document.getElementById('modelOutput');
    const processingIndicator = document.getElementById('processingIndicator');
    
    // Temperature slider sync
    const temperatureSlider = document.getElementById('temperatureSlider');
    const temperatureInput = document.getElementById('temperature');
    
    temperatureSlider.addEventListener('input', (e) => {
        temperatureInput.value = (e.target.value / 100).toFixed(1);
    });
    
    temperatureInput.addEventListener('input', (e) => {
        temperatureSlider.value = Math.round(parseFloat(e.target.value) * 100);
    });
    
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

    // Run Mamba-2 Model with Elite UX
    runModelBtn.addEventListener('click', async () => {
        const input = modelInput.value.trim();
        if (!input) {
            showToast('Please enter a prompt', 'error');
            return;
        }

        const stateDim = parseInt(document.getElementById('stateDim').value) || 128;
        const inputDim = parseInt(document.getElementById('inputDim').value) || 128;
        const temperature = parseFloat(document.getElementById('temperature').value) || 0.0;
        const frozenSeed = document.getElementById('frozenSeed').checked;

        // Elite loading state
        runModelBtn.disabled = true;
        runModelBtn.innerHTML = `
            <svg class="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Processing...
        `;
        processingIndicator.classList.remove('hidden');
        modelOutput.innerHTML = '<div class="flex items-center justify-center h-full"><div class="text-center"><div class="animate-spin w-8 h-8 border-4 border-miami-red border-t-transparent rounded-full mx-auto mb-4"></div><p class="text-white/50">Running Mamba-2 model...</p></div></div>';

        try {
            const result = await invoke('run_mamba_model', {
                prompt: input,
                stateDim,
                inputDim,
                temperature,
                frozenSeed
            });

            // Elite output display
            const outputHtml = `
                <div class="space-y-4 animate-fadeIn">
                    <div class="flex items-start gap-4">
                        <div class="flex-shrink-0 w-2 h-2 bg-miami-red rounded-full mt-2 animate-pulse"></div>
                        <div class="flex-1">
                            <div class="text-green-400 font-semibold mb-2 flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                Model Output
                            </div>
                            <pre class="text-white whitespace-pre-wrap leading-relaxed">${escapeHtml(result.output || result)}</pre>
                        </div>
                    </div>
                    ${result.metrics ? `
                        <div class="mt-6 pt-6 border-t border-miami-red/20">
                            <div class="text-miami-red font-semibold mb-3 flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"></path>
                                </svg>
                                Stability Metrics
                            </div>
                            <div class="bg-axiom-gray-100/30 rounded-lg p-4">
                                <pre class="text-white/80 text-xs">${escapeHtml(JSON.stringify(result.metrics, null, 2))}</pre>
                            </div>
                        </div>
                    ` : ''}
                </div>
            `;
            
            modelOutput.innerHTML = outputHtml;
            modelOutput.scrollTop = 0;

            // Update risk score if available
            if (result.riskScore !== undefined) {
                updateRiskScore(result.riskScore);
            }
            
            showToast('Model execution complete', 'success');
        } catch (error) {
            modelOutput.innerHTML = `
                <div class="flex items-center gap-3 text-red-400">
                    <svg class="w-5 h-5 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                    <div>
                        <div class="font-semibold">Error</div>
                        <div class="text-sm text-white/70">${escapeHtml(error)}</div>
                    </div>
                </div>
            `;
            showToast('Model execution failed', 'error');
        } finally {
            runModelBtn.disabled = false;
            runModelBtn.innerHTML = `
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"></path>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                </svg>
                Run Model
            `;
            processingIndicator.classList.add('hidden');
        }
    });

    // Clear Output
    clearBtn.addEventListener('click', () => {
        modelInput.value = '';
        modelOutput.innerHTML = '<div class="flex items-center justify-center h-full text-white/30"><div class="text-center"><svg class="w-12 h-12 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"></path></svg><p>Output will appear here...</p></div></div>';
        showToast('Cleared', 'info');
    });

    // Reset Model
    resetModelBtn.addEventListener('click', () => {
        document.getElementById('stateDim').value = '128';
        document.getElementById('inputDim').value = '128';
        document.getElementById('temperature').value = '0.0';
        document.getElementById('temperatureSlider').value = '0';
        document.getElementById('frozenSeed').checked = true;
        showToast('Model parameters reset', 'info');
    });

    // Parse TOON Data
    parseToonBtn.addEventListener('click', async () => {
        const input = toonInput.value.trim();
        if (!input) {
            showToast('Please enter TOON data', 'error');
            return;
        }

        parseToonBtn.disabled = true;
        parseToonBtn.textContent = 'Parsing...';
        toonOutput.textContent = '';

        try {
            const result = await invoke('parse_toon_data', { data: input });
            toonOutput.innerHTML = `
                <div class="text-green-400 mb-1 flex items-center gap-1">
                    <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                    </svg>
                    Parsed successfully
                </div>
                <pre class="text-white/70 text-xs overflow-auto">${escapeHtml(result)}</pre>
            `;
            showToast('TOON parsed successfully', 'success');
        } catch (error) {
            toonOutput.innerHTML = `<div class="text-red-400 text-xs">Error: ${escapeHtml(error)}</div>`;
            showToast('TOON parsing failed', 'error');
        } finally {
            parseToonBtn.disabled = false;
            parseToonBtn.textContent = 'Parse';
        }
    });

    // Verify Risk
    verifyRiskBtn.addEventListener('click', async () => {
        verifyRiskBtn.disabled = true;
        verifyRiskBtn.textContent = 'Verifying...';
        riskOutput.textContent = '';

        try {
            const result = await invoke('calculate_risk', { 
                input: 'AxiomHive_Sovereign_Manifold_v2.1.0' 
            });
            
            riskOutput.innerHTML = `
                <div class="space-y-1">
                    <pre class="text-white/70 text-xs whitespace-pre-wrap">${escapeHtml(result)}</pre>
                </div>
            `;

            // Parse and update UI
            const riskMatch = result.match(/Risk Score: (\d+)/);
            const entropyMatch = result.match(/Entropy Count: (\d+)/);
            
            if (riskMatch) {
                updateRiskScore(parseInt(riskMatch[1]));
            }
            if (entropyMatch) {
                document.getElementById('entropyCount').textContent = entropyMatch[1];
            }
            
            showToast('Risk verification complete', 'success');
        } catch (error) {
            riskOutput.innerHTML = `<div class="text-red-400 text-xs">Error: ${escapeHtml(error)}</div>`;
            showToast('Risk verification failed', 'error');
        } finally {
            verifyRiskBtn.disabled = false;
            verifyRiskBtn.textContent = 'Verify System';
        }
    });

    // FHE Encrypt
    encryptBtn.addEventListener('click', async () => {
        const input = parseInt(fheInput.value);
        if (isNaN(input) || input < 0 || input > 65535) {
            showToast('Please enter a valid number (0-65535)', 'error');
            return;
        }

        encryptBtn.disabled = true;
        encryptBtn.textContent = '...';
        fheOutput.textContent = '';

        try {
            const result = await invoke('encrypt_fhe', { message: input });
            encryptedData = result.ciphertext;
            fheKeys = result.keys;
            fheOutput.innerHTML = `
                <div class="text-green-400 mb-1 text-xs">Encrypted</div>
                <div class="text-white/70 text-xs break-all">${result.ciphertext.substring(0, 80)}...</div>
            `;
            showToast('Encryption successful', 'success');
        } catch (error) {
            fheOutput.innerHTML = `<div class="text-red-400 text-xs">Error: ${escapeHtml(error)}</div>`;
            showToast('Encryption failed', 'error');
        } finally {
            encryptBtn.disabled = false;
            encryptBtn.textContent = 'Encrypt';
        }
    });

    // FHE Decrypt
    decryptBtn.addEventListener('click', async () => {
        if (!encryptedData || !fheKeys) {
            showToast('Please encrypt data first', 'error');
            return;
        }

        decryptBtn.disabled = true;
        decryptBtn.textContent = '...';
        fheOutput.textContent = '';

        try {
            const result = await invoke('decrypt_fhe', { 
                ciphertext: encryptedData,
                keys: fheKeys
            });
            fheOutput.innerHTML = `
                <div class="text-green-400 mb-1 text-xs">Decrypted</div>
                <div class="text-white font-semibold">${result}</div>
            `;
            showToast('Decryption successful', 'success');
        } catch (error) {
            fheOutput.innerHTML = `<div class="text-red-400 text-xs">Error: ${escapeHtml(error)}</div>`;
            showToast('Decryption failed', 'error');
        } finally {
            decryptBtn.disabled = false;
            decryptBtn.textContent = 'Decrypt';
        }
    });

    // Initialize system status
    await updateSystemStatus();
});

function updateSystemTime() {
    const now = new Date();
    const timeStr = now.toLocaleTimeString('en-US', { 
        hour12: false,
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
    });
    document.getElementById('systemTime').textContent = timeStr;
}

function updateRiskScore(score) {
    const riskScoreEl = document.getElementById('riskScore');
    riskScoreEl.textContent = score;
    riskScoreEl.className = score === 0 
        ? 'text-green-400 font-mono font-bold' 
        : 'text-red-400 font-mono font-bold';
}

async function updateSystemStatus() {
    try {
        const status = await invoke('get_system_status');
        // Status already displayed in UI
    } catch (error) {
        console.error('Failed to get system status:', error);
    }
}

function showToast(message, type = 'info') {
    const container = document.getElementById('toastContainer');
    const toast = document.createElement('div');
    
    const colors = {
        success: 'bg-green-500/20 border-green-500/50 text-green-400',
        error: 'bg-red-500/20 border-red-500/50 text-red-400',
        info: 'bg-blue-500/20 border-blue-500/50 text-blue-400'
    };
    
    toast.className = `${colors[type]} border rounded-lg px-4 py-3 backdrop-blur-xl animate-slideInRight shadow-xl min-w-[300px]`;
    toast.innerHTML = `
        <div class="flex items-center gap-2">
            <div class="flex-1 font-medium text-sm">${escapeHtml(message)}</div>
            <button onclick="this.parentElement.parentElement.remove()" class="text-white/50 hover:text-white">
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                </svg>
            </button>
        </div>
    `;
    
    container.appendChild(toast);
    
    setTimeout(() => {
        toast.style.animation = 'slideOutRight 0.3s ease-out';
        setTimeout(() => toast.remove(), 300);
    }, 3000);
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}
