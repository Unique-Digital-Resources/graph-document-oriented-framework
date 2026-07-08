export class Viewport3D extends HTMLElement {
  connectedCallback() {
    if (!this.scene) this.initThree();
    this.render3D();
  }

  update(nodeData) {
    this._data = nodeData;
    if (this.isConnected) {
      this.render3D();
    }
  }

  initThree() {
    this.scene = new THREE.Scene();
    let w = this.clientWidth || 800;
    let h = this.clientHeight || 600;
    
    this.camera = new THREE.PerspectiveCamera(75, w / h, 0.1, 1000);
    this.camera.position.z = 5;
    this.renderer = new THREE.WebGLRenderer({ antialias: true });
    this.renderer.setSize(w, h);
    this.appendChild(this.renderer.domElement);
  }

  render3D() {
    if (!this.scene) return;
    
    const sceneJson = JSON.parse(window.wasm.get_3d_scene());
    const meshData = sceneJson.mesh;
    if (!meshData) return;
    
    if (this.cube) this.scene.remove(this.cube);

    let materials = meshData.faces.map(f => {
      let c = f.color.v; 
      return new THREE.MeshBasicMaterial({ color: new THREE.Color(c[0].v, c[1].v, c[2].v) });
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