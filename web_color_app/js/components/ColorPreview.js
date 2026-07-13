export class ColorPreview extends HTMLElement {
  update(nodeData) {
    this._data = nodeData;
    const props = nodeData.props || nodeData.data || {};
    const color = Array.isArray(props.value) ? props.value : [0, 100, 50, 1];
    const hex = props.hex || '#FF0000';
    const textColor = color[2] > 55 ? 'rgba(0,0,0,0.8)' : 'rgba(255,255,255,0.9)';

    if (!this.querySelector('.info-title')) {
      this.className = 'color-info-box';
      this.innerHTML = `<div class="info-title"></div><div class="info-subtitle"></div>`;
      this.querySelector('.info-title').addEventListener('dblclick', () => this.startEditHex());
    }

    this.style.backgroundColor = `hsla(${color[0]}, ${color[1]}%, ${color[2]}%, ${color[3]})`;
    this.style.color = textColor;
    
    if (!this._isEditing) {
      this.querySelector('.info-title').textContent = hex;
      this.querySelector('.info-subtitle').textContent = `hsla(${Math.round(color[0])}, ${Math.round(color[1])}%, ${Math.round(color[2])}%, ${color[3].toFixed(2)})`;
    }
  }

  startEditHex() {
    this._isEditing = true;
    const titleEl = this.querySelector('.info-title');
    if (!titleEl) return;
    
    const currentHex = titleEl.textContent;
    const input = document.createElement('input');
    input.type = 'text'; input.value = currentHex;
    input.style.cssText = `font-size: 20px; font-weight: bold; text-align: center; width: 100px; background: transparent; border: 1px solid #666; border-radius: 4px; color: inherit;`;
    titleEl.replaceWith(input);
    input.focus(); input.select();

    const commit = () => {
      this._isEditing = false;
      this.innerHTML = ''; 
      const targetNode = this._data?.props?.target_node || this._data?.data?.target_node;
      window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
        type: 'custom', target: this.id, command_id: 'SetDirectHexColor',
        params: { target_node: targetNode, hex: input.value }
      }}));
    };
    input.addEventListener('blur', commit);
    input.addEventListener('keydown', (e) => {
      if (e.key === 'Enter') commit();
      if (e.key === 'Escape') { this._isEditing = false; this.innerHTML = ''; this.update(this._data); }
    });
  }
}
customElements.define('color-preview', ColorPreview);