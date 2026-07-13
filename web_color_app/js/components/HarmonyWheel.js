import { CircleWheel } from './CircleWheel.js';

export class HarmonyWheel extends CircleWheel {
  render() {
    this.innerHTML = `
      <div class="cw-bg"></div>
      <div class="cw-thumb"></div>
      <svg class="cw-lines" style="position:absolute; top:0; left:0; width:100%; height:100%; pointer-events: none;"></svg>
      <svg class="cw-ruler" style="position:absolute; top:0; left:0; width:100%; height:100%; pointer-events: none; display:none;"></svg>
    `;
    this.bgEl = this.querySelector('.cw-bg');
    this.thumbEl = this.querySelector('.cw-thumb');
    this.linesSvg = this.querySelector('.cw-lines');
    this.rulerSvg = this.querySelector('.cw-ruler');
  }

  update(nodeData) {
    this._data = nodeData;
    const props = nodeData.props || {};
    const colors = Array.isArray(props.harmony_colors) ? props.harmony_colors : [];
    const rulerEnabled = props.ruler_enabled || false;
    const rulerMode = props.ruler_mode || 'lines';

    if (colors.length > 0) this.updateBackground(colors[0]);

    this.querySelectorAll('.cw-harm-thumb').forEach(t => t.remove());
    if (this.linesSvg) this.linesSvg.innerHTML = '';

    colors.forEach((c, i) => {
      const { x, y } = this.getCoordsFromColor(c, 50, 100);
      
      const line = document.createElementNS('http://www.w3.org/2000/svg', 'line');
      line.setAttribute('x1', '50');
      line.setAttribute('y1', '50');
      line.setAttribute('x2', x);
      line.setAttribute('y2', y);
      line.setAttribute('stroke', 'rgba(255,255,255,0.4)');
      line.setAttribute('stroke-width', '1');
      this.linesSvg.appendChild(line);

      const thumb = document.createElement('div');
      thumb.className = 'cw-harm-thumb cw-thumb';
      thumb.style.cssText = `position:absolute; width:14px; height:14px; border:2px solid #fff; border-radius:50%; pointer-events:auto; transform:translate(-50%, -50%); cursor:grab; background:hsla(${c[0]},${c[1]}%,${c[2]}%,${c[3]}); left:${x}%; top:${y}%;`;
      thumb.dataset.index = i;
      
      thumb.addEventListener('mousedown', (e) => this.startHarmonyDrag(e, i));
      thumb.addEventListener('touchstart', (e) => this.startHarmonyDrag(e, i), { passive: false });

      this.appendChild(thumb);
    });

    if (this.rulerSvg) {
      this.rulerSvg.style.display = rulerEnabled ? 'block' : 'none';
    }
  }

  startHarmonyDrag(e, index) {
    e.stopPropagation();
    this._isDragging = true;
    this._draggingThumbIndex = index;

    const onMove = (ev) => {
      if (!this._isDragging) return;
      ev.preventDefault();
      const rect = this.getBoundingClientRect();
      const cx = (ev.touches ? ev.touches[0].clientX : ev.clientX) - rect.left;
      const cy = (ev.touches ? ev.touches[0].clientY : ev.clientY) - rect.top;
      
      const color = this.getColorFromCoords(cx, cy, rect.width, rect.height);
      window.dispatchEvent(new CustomEvent('gdf-input', { detail: {
        type: 'custom', target: this.id, command_id: 'SetHarmonyColors',
        params: { target_node: this._data.props.target_node, colors: [color] }
      }}));
    };

    const onUp = () => {
      this._isDragging = false;
      window.removeEventListener('mousemove', onMove);
      window.removeEventListener('mouseup', onUp);
      window.removeEventListener('touchmove', onMove);
      window.removeEventListener('touchend', onUp);
    };

    window.addEventListener('mousemove', onMove);
    window.addEventListener('mouseup', onUp);
    window.addEventListener('touchmove', onMove, { passive: false });
    window.addEventListener('touchend', onUp);
  }
}

customElements.define('harmony-wheel', HarmonyWheel);