import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

export class Viewport3D extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        
        this.shadowRoot.innerHTML = `
            <style>
                :host { display: block; width: 100%; height: 100%; }
                canvas { 
                    display: block; 
                    width: 100%; 
                    height: 100%; 
                    outline: none;
                    cursor: grab;
                    touch-action: none;
                }
                canvas:active { cursor: grabbing; }
            </style>
            <canvas id="three-canvas" tabindex="0"></canvas>
        `;

        this.canvas = this.shadowRoot.getElementById('three-canvas');
        this.cameraId = null;
        this.isDragging = false;

        this._initThree();
        this._setupListeners();
    }

    _initThree() {
        // 1. Core Three.js Setup
        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(0x1a1a1a);

        this.camera = new THREE.PerspectiveCamera(45, 1, 0.1, 1000);
        this.camera.position.set(5, 5, 5);
        this.camera.lookAt(0, 0, 0);

        this.renderer = new THREE.WebGLRenderer({ canvas: this.canvas, antialias: true });

        // 2. Add the Cube Mesh
        const geometry = new THREE.BoxGeometry(1, 1, 1);
        const material = new THREE.MeshBasicMaterial({ vertexColors: true });
        
        const colors = [];
        const colorPalette = [
            [1, 0, 0], [0, 1, 0], [0, 0, 1], [1, 1, 0], [1, 0, 1], [0, 1, 1]
        ];
        for (let i = 0; i < 6; i++) {
            const c = colorPalette[i];
            for (let j = 0; j < 4; j++) {
                colors.push(c[0], c[1], c[2]);
            }
        }
        geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));
        
        this.cube = new THREE.Mesh(geometry, material);
        this.cube.position.y = 0.5; // Slightly above grid
        this.scene.add(this.cube);

        // 3. Procedural Infinite Grid (Gradual LOD & Adjusted Horizon)
        const gridGeometry = new THREE.PlaneGeometry(2, 2);
        const gridMaterial = new THREE.ShaderMaterial({
            uniforms: {
                invViewProj: { value: new THREE.Matrix4() },
                cameraDistance: { value: 1.0 } // Zoom level uniform
            },
            vertexShader: `
                varying vec2 vUv;
                void main() {
                    vUv = uv;
                    gl_Position = vec4(position.xy, 1.0, 1.0); // Full screen quad
                }
            `,
            fragmentShader: `
                varying vec2 vUv;
                uniform mat4 invViewProj;
                uniform float cameraDistance;

                float gridLine(float coord, float size, float thickness) {
                    float r = coord / size;
                    float grid = abs(fract(r - 0.5) - 0.5) / fwidth(r);
                    float line = 1.0 - min(grid / thickness, 1.0);
                    return line;
                }

                void main() {
                    vec2 ndc = vUv * 2.0 - 1.0;
                    vec4 rayWorld = invViewProj * vec4(ndc, 1.0, 1.0);
                    rayWorld.xyz /= rayWorld.w; 
                    
                    vec3 ray = normalize(rayWorld.xyz - cameraPosition);
                    float planeY = 0.0;
                    float t = (planeY - cameraPosition.y) / ray.y; 
                    
                    if (t <= 0.0) discard;

                    vec3 pos = cameraPosition + t * ray;
                    float dist = length(pos - cameraPosition);

                    // 3-Tier Gradual Level of Detail 
                    // Tier 1: 1m grid (Minor)
                    float l1 = max(gridLine(pos.x, 1.0, 1.0), gridLine(pos.z, 1.0, 1.0));
                    float l1_a = l1 * (1.0 - smoothstep(15.0, 40.0, cameraDistance)) * 0.5;

                    // Tier 2: 10m grid (Major for Tier 1, Minor for Tier 3)
                    float l2 = max(gridLine(pos.x, 10.0, 1.5), gridLine(pos.z, 10.0, 1.5));
                    float l2_a = l2 * (1.0 - smoothstep(120.0, 250.0, cameraDistance)) * 0.8;

                    // Tier 3: 100m grid (Major for Tier 2)
                    float l3 = max(gridLine(pos.x, 100.0, 2.0), gridLine(pos.z, 100.0, 2.0));
                    float l3_a = l3 * (1.0 - smoothstep(800.0, 2000.0, cameraDistance)) * 1.0;

                    // Combine grid levels
                    float gridAlpha = max(l1_a, max(l2_a, l3_a));
                    vec3 gridColor = vec3(0.7);

                    // World Axes (X = Red, Z = Green)
                    float axisX = 1.0 - min(abs(pos.z) / (fwidth(pos.z) * 2.0), 1.0);
                    float axisZ = 1.0 - min(abs(pos.x) / (fwidth(pos.x) * 2.0), 1.0);
                    
                    vec3 axisColor = vec3(0.0);
                    axisColor = mix(axisColor, vec3(0.9, 0.2, 0.2), axisX); 
                    axisColor = mix(axisColor, vec3(0.2, 0.9, 0.2), axisZ); 
                    
                    float axisAlpha = max(axisX, axisZ);
                    
                    // Blend axes over grid
                    vec3 finalColor = mix(gridColor, axisColor, axisAlpha);
                    float finalAlpha = max(gridAlpha, axisAlpha);
                    
                    // Adjusted Horizon Fog (Brought closer)
                    finalAlpha *= 1.0 - smoothstep(150.0, 700.0, dist);

                    gl_FragColor = vec4(finalColor, finalAlpha);
                }
            `,
            transparent: true,
            depthWrite: false
        });
        this.grid = new THREE.Mesh(gridGeometry, gridMaterial);
        this.grid.frustumCulled = false; 
        this.scene.add(this.grid);

        // 4. Initialize Orbit Controls
        this.controls = new OrbitControls(this.camera, this.canvas);
        this.controls.enableDamping = true;
        this.controls.dampingFactor = 0.1;
        
        // Zoom Limits
        this.controls.minDistance = 2;
        this.controls.maxDistance = 250;
        
        this.controls.mouseButtons = {
            LEFT: THREE.MOUSE.ROTATE,
            MIDDLE: THREE.MOUSE.ROTATE,
            RIGHT: THREE.MOUSE.PAN
        };

        // 5. Resize Handler
        this.resizeObserver = new ResizeObserver(() => this._handleResize());
        this.resizeObserver.observe(this.canvas);
        this._handleResize();

        // 6. Render Loop
        this._animate();
    }

    _setupListeners() {
        // Track Shift key to enable exact Blender-style MMB panning
        window.addEventListener('keydown', (e) => {
            if (e.key === 'Shift') this.controls.mouseButtons.MIDDLE = THREE.MOUSE.PAN;
        });
        window.addEventListener('keyup', (e) => {
            if (e.key === 'Shift') this.controls.mouseButtons.MIDDLE = THREE.MOUSE.ROTATE;
        });

        // STRICTLY prevent browser default actions on the canvas
        this.canvas.addEventListener('contextmenu', (e) => e.preventDefault());
        
        // Prevent page scrolling on wheel
        this.canvas.addEventListener('wheel', (e) => e.preventDefault(), { passive: false });
        
        // Prevent text/image dragging on mousedown
        this.canvas.addEventListener('mousedown', (e) => {
            e.preventDefault();
            this.canvas.focus();
        });

        // Track interaction for GDF Graph committing
        this.controls.addEventListener('start', () => { this.isDragging = true; });
        this.controls.addEventListener('end', () => {
            this.isDragging = false;
            this._emitEvent('gdf-change', { interaction_type: 'release' });
        });

        this.controls.addEventListener('change', () => {
            if (this.isDragging) {
                this._emitEvent('gdf-input', {
                    position: [this.camera.position.x, this.camera.position.y, this.camera.position.z],
                    target: [this.controls.target.x, this.controls.target.y, this.controls.target.z]
                });
            }
        });
    }

    _handleResize() {
        const width = this.canvas.clientWidth;
        const height = this.canvas.clientHeight;
        if (width > 0 && height > 0) {
            this.renderer.setSize(width, height, false);
            this.camera.aspect = width / height;
            this.camera.updateProjectionMatrix();
        }
    }

    _animate() {
        requestAnimationFrame(() => this._animate());
        this.controls.update();
        
        // Update inverse view-projection matrix
        this.camera.updateMatrixWorld();
        const viewMatrix = this.camera.matrixWorldInverse;
        const projMatrix = this.camera.projectionMatrix;
        
        this.grid.material.uniforms.invViewProj.value.multiplyMatrices(projMatrix, viewMatrix).invert();
        
        // Pass absolute zoom level (distance to target) to the shader
        this.grid.material.uniforms.cameraDistance.value = this.camera.position.distanceTo(this.controls.target);

        this.renderer.render(this.scene, this.camera);
    }

    update(nodeData) {
        if (nodeData.props && nodeData.props.data) {
            try {
                const data = JSON.parse(nodeData.props.data);
                this.cameraId = data.camera_id;
            } catch (e) {
                console.error("Failed to parse viewport data binding", e);
            }
        }
    }

    _emitEvent(eventName, payload) {
        if (!this.cameraId) return;

        this.dispatchEvent(new CustomEvent(eventName, {
            bubbles: true,
            composed: true,
            detail: {
                command: "UpdateViewportCamera",
                params: {
                    camera_id: this.cameraId,
                    ...payload
                }
            }
        }));
    }
}

customElements.define('viewport-3d', Viewport3D);