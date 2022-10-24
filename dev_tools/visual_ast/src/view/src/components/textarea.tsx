import { TextField } from '@mui/material';
import { ChangeEvent, useState } from 'react';

const Textarea = () => {
  const [value, setValue] = useState('');

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    setValue(e.target.value);
    // TODO
  };

  return (
    <>
      <TextField
        label='code'
        rows={35}
        value={value}
        onChange={handleChange}
        fullWidth
        multiline
      />
    </>
  );
};

export default Textarea;
