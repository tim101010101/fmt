import type { TreeSeriesOption } from 'echarts';

export interface TreeData {
  name: string;
  children: TreeSeriesOption['data'];
}
