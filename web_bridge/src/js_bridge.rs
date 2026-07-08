//! Serves the JavaScript bridge code to the frontend.
//! 
//! The frontend runs this script to listen for JSON updates from the Rust
//! backend and apply them to the actual browser DOM, as well as forwarding
//! browser events back to the backend.

pub const JS_BRIDGE_CODE: &str = r#"
class GdfBridge {
    constructor(wsUrl) {
        this.ws = new WebSocket(wsUrl);
        this.rootElement = document.getElementById('gdf-root');
        
        this.ws.onmessage = (event) => this.handleMessage(JSON.parse(event.data));
        
        // Bind global event listeners
        document.addEventListener('click', (e) => this.forwardEvent('click', e));
        document.addEventListener('input', (e) => this.forwardEvent('input', e));
        document.addEventListener('keydown', (e) => this.forwardEvent('keydown', e));
    }

    handleMessage(msg) {
        if (msg.type === 'dom_update') {
            this.applyDom(msg.payload);
        }
    }

    applyDom(node) {
        // Minimal Virtual DOM diff/apply logic
        let el = document.getElementById(node.id) || this.createElement(node);
        
        if (node.text) el.innerText = node.text;
        el.style.position = 'absolute';
        el.style.left = node.bounds[0] + 'px';
        el.style.top = node.bounds[1] + 'px';
        el.style.width = node.bounds[2] + 'px';
        el.style.height = node.bounds[3] + 'px';

        // Clear and rebuild children (naive approach)
        el.innerHTML = '';
        for (let child of node.children) {
            let childEl = this.createElement(child);
            el.appendChild(childEl);
        }
    }

    createElement(node) {
        let el = document.createElement(node.tag);
        el.id = node.id;
        return el;
    }

    forwardEvent(type, e) {
        let target = e.target.closest('[id]');
        if (!target || !target.id) return;
        
        let payload = { type: type, target: target.id };
        if (type === 'input') payload.value = target.value;
        if (type === 'keydown') payload.key = e.key;
        
        this.ws.send(JSON.stringify(payload));
    }
}
window.gdf = new GdfBridge('ws://localhost:8080');
"#;