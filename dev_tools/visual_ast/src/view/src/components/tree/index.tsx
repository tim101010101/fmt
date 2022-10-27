import { Tabs, Tab } from '@mui/material';
import { ReactNode, SyntheticEvent, useState } from 'react';
import { RenderTree } from '../../types/tree';
import TreeJSON from './treeJSON';
import TreeView from './treeView';

const getTreeData = (): RenderTree => {
  return {
    id: '1',
    name: 'hello',
    children: [
      { id: '2', name: 'world' },
      {
        id: '3',
        name: 'foo',
        children: [
          { id: '4', name: 'bar' },
          { id: '5', name: 'baz' },
        ],
      },
      {
        id: '6',
        name: 'foo1',
        children: [
          { id: '7', name: 'bar1' },
          { id: '8', name: 'baz1' },
        ],
      },
    ],
  };
};

interface TabPanelProps {
  children?: ReactNode;
  index: number;
  value: number;
}

const TabPanel = (props: TabPanelProps) => {
  const { children, value, index } = props;

  return <div>{value === index && <>{children}</>}</div>;
};

const Tree = () => {
  const [value, setValue] = useState(0);

  const handleChange = (e: SyntheticEvent, newValue: number) => {
    setValue(newValue);
  };

  const root = getTreeData();

  return (
    <>
      <Tabs value={value} onChange={handleChange} variant='fullWidth'>
        <Tab label='JSON'></Tab>
        <Tab label='VISUAL'></Tab>
      </Tabs>

      <TabPanel index={value} value={0}>
        <TreeJSON root={root} />
      </TabPanel>
      <TabPanel index={value} value={1}>
        <TreeView root={root} />
      </TabPanel>
    </>
  );
};

export default Tree;
