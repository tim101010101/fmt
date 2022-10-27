export interface Token {
  kind: string;
  text: string;
}

export interface Node {
  kind: string;
  len: number;
  children: Array<Node | Token>;
}
