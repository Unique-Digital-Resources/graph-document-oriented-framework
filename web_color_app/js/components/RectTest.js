export class RectTest extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  update(nodeData) {
    this._data = nodeData;
    const props = nodeData.props || {};
    const color = Array.isArray(props.color) ? props.color : [0, 100, 50, 1];
    
    if (!this.shadowRoot.innerHTML) {
      this.shadowRoot.innerHTML = `
        <style>
          :host { display: block; width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; }
          .rect { width: 80%; aspect-ratio: 4/3; border-radius: 12px; box-shadow: 0 10px 30px rgba(0,0,0,0.5); transition: background 0.1s; }
        </style>
        <div class="rect"></div>
      `;
    }
    
    this.shadowRoot.querySelector('.rect').style.backgroundColor = `hsla(${color[0]}, ${color[1]}%, ${color[2]}%, ${color[3]})`;
  }
}

customElements.define('rect-test', RectTest);