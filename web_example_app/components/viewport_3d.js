export class Viewport3D extends HTMLElement {
  connectedCallback() {
    if (!this.scene) {
      this.initThree();
    }
  }

  update(nodeData) {
    this._data = nodeData;
    if (this.scene) {
      this.render3D();
    }
  }

  initThree() {
    this.scene = new THREE.Scene();
    this.camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
    this.camera.position.z = 5;
    this.renderer = new THREE.WebGLRenderer({ antialias: true });
    this.appendChild(this.renderer.domElement);

    // ResizeObserver fires exactly when the CSS layout gives this element its dimensions
    const resizeObserver = new ResizeObserver(() => {
      let w = this.clientWidth;
      let h = this.clientHeight;
      if (w > 0 && h > 0) {
        this.camera.aspect = w / h;
        this.camera.updateProjectionMatrix();
        this.renderer.setSize(w, h);
        this.render3D(); // Render immediately once sized
      }
    });
    resizeObserver.observe(this);
  }

  render3D() {
    if (!this.scene || !this.renderer) return;
    
    const sceneJson = JSON.parse(window.wasm.get_3d_scene());
    const meshData = sceneJson.mesh;
    if (!meshData) return;
    
    if (this.cube) this.scene.remove(this.cube);

    let colors = meshData.face_colors.v;
    let materials = colors.map(c => {
      let col = c.v; 
      return new THREE.MeshBasicMaterial({ color: new THREE.Color(col[0].v, col[1].v, col[2].v) });
    });

    this.cube = new THREE.Mesh(new THREE.BoxGeometry(), materials);
    
    let p = meshData.position.v;
    let r = meshData.rotation.v;
    let s = meshData.scale.v;
    
    this.cube.position.set(p[0].v, p[1].v, p[2].v);
    this.cube.rotation.set(r[0].v, r[1].v, r[2].v);
    this.cube.scale.set(s[0].v, s[1].v, s[2].v);
    
    this.scene.add(this.cube);
    this.renderer.render(this.scene, this.camera);
  }
}
customElements.define('viewport-3d', Viewport3D);