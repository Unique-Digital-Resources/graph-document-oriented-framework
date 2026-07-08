export class GdfNode extends HTMLElement {
  update(nodeData) {
    this._data = nodeData;
    this.render();
  }
  render() {
    this.style.width = this._data.bounds[2] + 'px';
    this.style.height = this._data.bounds[3] + 'px';
    if (this._data.tag === 'div' && this._data.children.length > 0) {
      this.className = 'gdf-inspector';
    }
  }
}
customElements.define('gdf-node', GdfNode);