import * as echarts from 'echarts';
import type { ECharts } from 'echarts';
import { useEffect, useRef, useState } from 'react';
import { ChartProps } from './types';

const Chart = ({ options }: ChartProps) => {
  const chartRef = useRef<HTMLDivElement>(null);
  const [chart, setChart] = useState<ECharts | null>(null);

  const resizeChart = () => {
    chart?.resize();
  };

  useEffect(() => {
    setChart(echarts.init(chartRef.current!));
  }, []);

  useEffect(() => {
    if (chart) {
      window.addEventListener('resize', resizeChart);
      return () => {
        window.removeEventListener('resize', resizeChart);
        chart.dispose();
        setChart(null);
      };
    }
  }, [chart]);

  useEffect(() => {
    chart && chart.setOption(options);
  }, [chart, options]);

  return <div ref={chartRef} style={{ width: '100%', height: '790px' }}></div>;
};

export default Chart;
