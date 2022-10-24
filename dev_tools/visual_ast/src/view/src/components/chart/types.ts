import type { EChartsOption } from 'echarts';
import { TreeData } from '../../types/treeData';

export interface ChartProps {
  options: EChartsOption;
}

export interface TreeChartProps {
  data: TreeData;
}
