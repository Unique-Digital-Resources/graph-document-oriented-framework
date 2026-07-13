import { Viewport3D } from './js/components/viewport_3d.js';

class GdfApp {
    constructor() {
        this.viewportComponent = null;
    }

    async init() {
        console.log("Initializing Three.js GDF App...");

        const mountPoint = document.getElementById('viewport-mount');
        this.viewportComponent = document.createElement('viewport-3d');
        mountPoint.appendChild(this.viewportComponent);

        this.setupBridge();
    }

    setupBridge() {
        // Listen for gdf-change (mouse release) to commit final state to GDF Graph
        document.addEventListener('gdf-change', (e) => {
            const { params } = e.detail;
            if (params.interaction_type === 'release') {
                // In a fully wired app, this would send an RPC to the Rust backend
                // to run the CommitViewportCamera command.
                console.log("Committing camera state to GDF Graph...", this.viewportComponent.camera.position);
            }
        });

        // Listen for gdf-input (dragging) for real-time updates
        document.addEventListener('gdf-input', (e) => {
            const { params } = e.detail;
            // console.log("Camera moved:", params.position);
        });
    }
}

window.addEventListener('DOMContentLoaded', () => {
    const app = new GdfApp();
    app.init();
});