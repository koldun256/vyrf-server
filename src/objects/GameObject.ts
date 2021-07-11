export interface Component {
  mount?(gameObject: GameObject<any[]>): void;
  update?(): void;
}
export type ComponentConstructor<T extends Component> = new (...args: any[]) => T

export abstract class GameObject<T extends Component[]> {
  constructor(
    public position: [number, number],
    private components: T
  ) { }

  public getComponent<C extends T[number]>(componentConstructor: ComponentConstructor<C>): C {
    return this.components.find(component => component instanceof componentConstructor) as C
  }
}
