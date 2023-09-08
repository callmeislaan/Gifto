import React from 'react';
import {
  Avatar,
  Box,
  ListItemAvatar,
  ListItemButton,
  Tooltip,
  Typography,
  Zoom,
  Link
} from '@mui/material';

const UserInfoRow = ({
  userInfo,
  mobileDevice,
}: {
  userInfo: any;
  mobileDevice: boolean;
}) => {
  if (mobileDevice) {
    return (
      <Link
        href={`/profile/${userInfo.username}`}
      >
        <Tooltip placement="top" title={userInfo.username}>
          <Zoom in>
            <div
              style={{
                width: 'max-content',
                height: 'max-content',
                border: '2px solid #057642',
                borderRadius: '50%',
              }}
            >
              <Avatar
                src={userInfo.headImgUrl}
                sx={{
                  border: '2px solid #fff',
                  borderRadius: '50%',
                  width: 54,
                  height: 54,
                  cursor: 'pointer',
                }}
              />
            </div>
          </Zoom>
        </Tooltip>
      </Link>
    );
  }
  return (
    <Link
      href={`/profile/${userInfo.username}`}
    >
      <ListItemButton>
        <ListItemAvatar>
          <Zoom in>
            <Avatar
              src={userInfo.headImgUrl}
              sx={{
                width: 40,
                height: 40,
              }}
            />
          </Zoom>
        </ListItemAvatar>
        <Link
          href={`/profile/${userInfo.username}`}
        >
          <Box
            sx={{
              display: 'flex',
              justifyContent: 'flex-end',
              textDecoration: 'none',
              color: '#65748B',
            }}
          >
            <Typography
              variant="subtitle1"
              sx={{ color: '#546378', marginLeft: '-2px' }}
            >
              {`${userInfo.username}`}
            </Typography>
          </Box>
        </Link>
      </ListItemButton>
    </Link>
  );
};

export default UserInfoRow;
