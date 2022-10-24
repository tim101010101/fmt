import { Grid, Card } from '@mui/material';
import Textarea from '../components/textarea';
import Tree from '../components/tree';

const Main = () => {
  return (
    <Grid container spacing={2}>
      <Grid item xs={4}>
        <Textarea />
      </Grid>
      <Grid item xs={8}>
        <Card variant='outlined'>
          <Tree />
        </Card>
      </Grid>
    </Grid>
  );
};

export default Main;
