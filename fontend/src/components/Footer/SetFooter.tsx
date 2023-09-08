import React, { useState, useEffect } from 'react';
import { useDispatch } from 'react-redux';
import { Button } from '@mui/material';
import CodeEditor from './CodeEditor';
import hotToast from '../../utils/hotToast';

interface Footer {
  id: string;
  name: string;
  code: string;
}

const SetFooter = () => {
  const dispatch = useDispatch();
  const [footer, setFooter] = useState<Footer>({ id: '', name: '', code: '' });
  const [footerCode, setFooterCode] = useState<string>('');

  return (
    <>
      <CodeEditor code={footerCode} setCode={setFooterCode} />
      <Button
        sx={{ mt: 2, mb: 2 }}
        variant="outlined"
        onClick={() => {
          hotToast('info', 'Nothing changed');
        }}
      >
        Submit Changes
      </Button>
    </>
  );
};

export default SetFooter;
