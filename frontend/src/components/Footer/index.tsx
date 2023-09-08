import React, { useEffect } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { Box, Container, Divider } from '@mui/material';
import { parser } from '../../utils/htmlParser';
import DefaultFooter from './DefaultFooter';

const Footer: React.FC = () => {
  const { footer } = useSelector((state: any) => state.sign);
  const dispatch = useDispatch();

  if (!footer) return null;
  return (
    <Container maxWidth="lg">
      <Box sx={{ mt: 8 }}>
        <Divider />
        {footer.code === '' ? <DefaultFooter /> : parser(footer.code)}
      </Box>
    </Container>
  );
};

export default Footer;
