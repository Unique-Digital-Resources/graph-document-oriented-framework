export class WheelMode extends HTMLElement {
  update(nodeData) {
    this._data = nodeData;
    const props = nodeData.props || nodeData.data || {};
    const mode = props.mode || 'Ranges'; // FIX: Fallback to Ranges

    if (!this.querySelector('.mode-btn')) {
      this.className = 'mode-toggles';
      this.innerHTML = `
        <button class="mode-btn" data-mode="WheelTriangle" title="Triangle"><span class="mdi mdi-triangle-outline"></span></button>
        <button class="mode-btn" data-mode="WheelSquare" title="Square"><span class="mdi mdi-square-outline"></span></button>
        <button class="mode-btn" data-mode="Circle" title="Circle"><span class="mdi mdi-circle-outline"></span></button>
        <button class="mode-btn" data-mode="Ranges" title="Ranges"><span class="mdi mdi-tune"></span></button>
      `;
      
      this.querySelectorAll('.mode-btn').forEach(btn => {
        btn.addEventListener('click', () => {
          window.dispatchEvent(new CustomEvent('gdf-change', { detail: {
            type: 'custom', target: this.id, command_id: 'SetWheelMode',
            params: { target_node: this.id, mode: btn.dataset.mode }
          }}));
        });
      });
    }

    this.querySelectorAll('.mode-btn').forEach(btn => {
      btn.classList.toggle('active', btn.dataset.mode === mode);
    });
  }
}
customElements.define('wheel-mode', WheelMode);