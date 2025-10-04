/* /public/script.js */

async function main() {

    function createElement(tag, properties = {}) {
        const el = document.createElement(tag);
        for (const [key, value] of Object.entries(properties)) {
            if (key === 'children' && Array.isArray(value)) {
                el.append(...value);
            } else {
                el[key] = value;
            }
        }
        return el;
    }

    function buildUI() {
        const pasteInput = createElement('textarea', { id: 'paste-input', placeholder: 'Paste your pokepaste here...', rows: 15 });
        const compressBtn = createElement('button', { id: 'compress-btn', textContent: 'Compress' });
        const formatSelect = createElement('select', { id: 'format-select', innerHTML: `<option value="base64" selected>Base64</option><option value="hex">Hex</option>` });
        const compressedOutput = createElement('textarea', { id: 'compressed-output', placeholder: 'Compressed output...', readOnly: true, rows: 12 });
        const copyCompressedBtn = createElement('button', { className: 'copy-btn', textContent: 'Copy' });
        const compressedInput = createElement('textarea', { id: 'compressed-input', placeholder: 'Paste your compressed string here...', rows: 15 });
        const decompressBtn = createElement('button', { id: 'decompress-btn', textContent: 'Decompress' });
        const decompressedOutput = createElement('textarea', { id: 'decompressed-output', placeholder: 'Decompressed pokepaste...', readOnly: true, rows: 12 });
        const copyDecompressedBtn = createElement('button', { className: 'copy-btn', textContent: 'Copy' });

        const container = createElement('div', { className: 'container', children: [
            createElement('header', { innerHTML: `<h1>PokéPack</h1><p>Compress and decompress Pokémon Showdown pastes with WebAssembly.</p>` }),
            createElement('div', { className: 'main-content', children: [
                createElement('div', { className: 'column', children: [
                    createElement('h2', { textContent: 'Compress' }),
                    pasteInput,
                    createElement('div', { className: 'controls', children: [compressBtn, formatSelect] }),
                    createElement('div', { className: 'output-wrapper', children: [compressedOutput, copyCompressedBtn] })
                ]}),
                createElement('div', { className: 'column', children: [
                    createElement('h2', { textContent: 'Decompress' }),
                    compressedInput,
                    createElement('div', { className: 'controls', children: [decompressBtn] }),
                    createElement('div', { className: 'output-wrapper', children: [decompressedOutput, copyDecompressedBtn] })
                ]})
            ]}),
            createElement('footer', { innerHTML: `<p>Compression Ratio: <span id="compression-ratio">N/A</span></p>` })
        ]});

        document.body.append(container);
        
        return { pasteInput, compressedOutput, compressBtn, formatSelect, copyCompressedBtn, compressedInput, decompressedOutput, decompressBtn, copyDecompressedBtn };
    }

    function copyToClipboard(textarea, button) {
        if (!textarea.value) return;
        navigator.clipboard.writeText(textarea.value).then(() => {
            const originalText = button.textContent;
            button.textContent = 'Copied!';
            button.classList.add('copied');
            setTimeout(() => {
                button.textContent = originalText;
                button.classList.remove('copied');
            }, 2000);
        }).catch(err => {
            console.error('Failed to copy text: ', err);
            alert('Failed to copy to clipboard.');
        });
    }
    
    const ui = buildUI();

    // --- Event Listeners ---
    ui.compressBtn.addEventListener('click', () => {
        const paste = ui.pasteInput.value;
        if (!paste.trim()) return;
        try {
            const result = (ui.formatSelect.value === 'base64')
                ? wasm_bindgen.pokepaste_to_base64(paste)
                : wasm_bindgen.pokepaste_to_hex(paste);
            ui.compressedOutput.value = result;
            const originalSize = new TextEncoder().encode(paste).length;
            const compressedSize = new TextEncoder().encode(result).length;
            document.getElementById('compression-ratio').textContent = `${(originalSize / compressedSize).toFixed(2)}:1`;
        } catch (e) {
            ui.compressedOutput.value = `Error: ${e}`;
        }
    });

    ui.decompressBtn.addEventListener('click', () => {
        const compressed = ui.compressedInput.value.trim();
        if (!compressed) return;
        try {
            ui.decompressedOutput.value = wasm_bindgen.hex_to_pokepaste(compressed);
        } catch (hexError) {
            try {
                ui.decompressedOutput.value = wasm_bindgen.base64_to_pokepaste(compressed);
            } catch (base64Error) {
                ui.decompressedOutput.value = "Error: Failed to decode. Input must be a valid Hex or Base64 string.";
            }
        }
    });

    ui.copyCompressedBtn.addEventListener('click', () => copyToClipboard(ui.compressedOutput, ui.copyCompressedBtn));
    ui.copyDecompressedBtn.addEventListener('click', () => copyToClipboard(ui.decompressedOutput, ui.copyDecompressedBtn));
}

document.addEventListener('DOMContentLoaded', main);
