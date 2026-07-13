export class ColorWheel extends HTMLElement {
  constructor() {
    super();
    this._isDragging = false;
    this._dragMode = null;
  }

  connectedCallback() {
    this.render();
    this.bindEvents();
    this._resizeObserver = new ResizeObserver(() => this._handleResize());
    this._resizeObserver.observe(this);
  }

  disconnectedCallback() {
    if (this._resizeObserver) this._resizeObserver.disconnect();
  }

  _handleResize() {
    if (this._data) this.update(this._data);
  }

  render() {
    this.innerHTML = `<div class="cw-container" style="width:100%; height:100%; position:relative;"></div>`;
    this.containerEl = this.querySelector('.cw-container');
  }

  // Helper to safely get target_node regardless of GDF serialization
  getTargetNode() {
    return this._data?.props?.target_node || this._data?.data?.target_node || this._data?.target_node;
  }

  update(nodeData) {
    this._data = nodeData;
    const props = nodeData.props || nodeData.data || {};
    this._color = Array.isArray(props.value) ? props.value : [0, 100, 50, 1];
    
    // FIX: Wait for the browser to provide a valid width before rendering.
    // This prevents the wheel from drawing at the wrong scale on initial load.
    const newSize = this.clientWidth;
    if (newSize > 0) this._size = newSize;
    
    if (!this._size || this._size <= 0) {
      requestAnimationFrame(() => this.update(nodeData));
      return;
    }
    
    this.updateBackground(this._color, this._size);
    this.updateThumbs(this._color, this._size);
  }

  bindEvents() {
    const onMove = (e) => {
      if (!this._isDragging) return;
      e.preventDefault();
      const rect = this.getBoundingClientRect();
      const size = rect.width;
      if (!size) return;
      
      const cx = (e.touches ? e.touches[0].clientX : e.clientX) - rect.left;
      const cy = (e.touches ? e.touches[0].clientY : e.clientY) - rect.top;
      const color = this.getColorFromCoords(cx, cy, size, this._dragMode);
      
      if (color) {
        const targetNode = this.getTargetNode();
        if (!targetNode) return; // Abort if no target node
        
        window.dispatchEvent(new CustomEvent('gdf-input', { detail: { type: 'custom', target: this.id, command_id: 'SetDirectColorProp', params: { target_node: targetNode, prop: "color", index: 0, value: color[0] } }}));
        window.dispatchEvent(new CustomEvent('gdf-input', { detail: { type: 'custom', target: this.id, command_id: 'SetDirectColorProp', params: { target_node: targetNode, prop: "color", index: 1, value: color[1] } }}));
        window.dispatchEvent(new CustomEvent('gdf-input', { detail: { type: 'custom', target: this.id, command_id: 'SetDirectColorProp', params: { target_node: targetNode, prop: "color", index: 2, value: color[2] } }}));
      }
    };

    const onDown = (e) => {
      this._isDragging = true;
      const rect = this.getBoundingClientRect();
      const size = rect.width;
      if (!size) { this._isDragging = false; return; }

      const cx = (e.touches ? e.touches[0].clientX : e.clientX) - rect.left;
      const cy = (e.touches ? e.touches[0].clientY : e.clientY) - rect.top;
      const center = size / 2;
      const outerR = center - 10;
      const innerR = outerR - Math.max(size * 0.08, 15);
      const d = Math.sqrt((cx - center) ** 2 + (cy - center) ** 2);

      if (d >= innerR && d <= outerR) this._dragMode = 'hue';
      else if (d < innerR) this._dragMode = 'picker';
      else { this._isDragging = false; return; }

      onMove(e);
    };

    const onUp = () => { this._isDragging = false; this._dragMode = null; };

    this.addEventListener('mousedown', onDown);
    this.addEventListener('touchstart', onDown, { passive: false });
    window.addEventListener('mousemove', onMove);
    window.addEventListener('touchmove', onMove, { passive: false });
    window.addEventListener('mouseup', onUp);
    window.addEventListener('touchend', onUp);
  }

  getCenter(size) { return size / 2; }
  getOuterRadius(size) { return this.getCenter(size) - 10; }
  getInnerRadius(size) { return this.getOuterRadius(size) - Math.max(size * 0.08, 15); }
  polarToCartesian(cx, cy, r, angleDeg) {
    const rad = (angleDeg - 90) * Math.PI / 180.0;
    return { x: cx + r * Math.cos(rad), y: cy + r * Math.sin(rad) };
  }

  updateBackground(color, size) {}
  updateThumbs(color, size) {}
  getColorFromCoords(x, y, size, mode) { return null; }
}
customElements.define('color-wheel', ColorWheel);