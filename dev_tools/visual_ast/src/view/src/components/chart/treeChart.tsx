import Chart from './chart';
import { TreeData } from '../../types/treeData';
import { ChartProps, TreeChartProps } from './types';

const getOption = (data: TreeData): ChartProps['options'] => {
  return {
    tooltip: {
      trigger: 'item',
      triggerOn: 'mousemove',
    },
    series: [
      {
        type: 'tree',
        data: [data],
        left: '2%',
        right: '2%',
        top: '8%',
        bottom: '20%',
        symbol: 'emptyCircle',
        orient: 'vertical',
        expandAndCollapse: true,
        edgeShape: 'polyline',
        label: {
          position: 'top',
          verticalAlign: 'middle',
          align: 'right',
          fontSize: 9,
        },
        leaves: {
          label: {
            position: 'bottom',
            verticalAlign: 'middle',
            align: 'left',
          },
        },
        animationDurationUpdate: 750,
      },
    ],
  };
};

export const TreeChart = ({ data }: TreeChartProps) => {
  const treeOptions = getOption(data);

  return (
    <>
      <Chart options={treeOptions} />
    </>
  );
};
