export type Component = Object;
export type ComponentConstructor<Component> = new (...args: any[]) => Component

export abstract class GameObject<T extends Component[]> {
  constructor(
    public position: [number, number],
    private components: T
  ) { }

  public getComponent<C extends T[number]>(componentConstructor: ComponentConstructor<C>): C {
    return this.components.find(component => component instanceof componentConstructor) as C
  }
}
