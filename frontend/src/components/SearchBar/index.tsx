import React, { useState, useEffect } from 'react';
import TextField from '@mui/material/TextField';
import List from '@mui/material/List';
import Badge from '@mui/material/Badge';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import Avatar from '@mui/material/Avatar';
import SearchIcon from '@mui/icons-material/Search';
import Autocomplete from '@mui/material/Autocomplete';
import { useSelector, useDispatch } from 'react-redux';
import { InputAdornment, Link } from '@mui/material';

const SearchBar: React.FC = () => {
  const { isLogin } = useSelector((state: any) => state.sign);
  const [searchInput, setSearchInput] = useState('');
  const dispatch = useDispatch();
  const [searchResults, setSearchResults] = useState([]);


  return (
    <Autocomplete
      id="custom-autocomplete"
      freeSolo
      options={searchResults}
      style={{ margin: 5 }}
      getOptionLabel={(option: any) => {
        if (option.title !== undefined) {
          return `${option.title}`;
        }
        if (option.username !== undefined) {
          return `${option.username}`;
        }
        return option;
      }}
      inputValue={searchInput}
      renderInput={(params: any) => {
        return (
          <TextField
            {...params}
            variant="outlined"
            label="Search"
            InputProps={{
              ...params.InputProps,
              startAdornment: (
                <>
                  <InputAdornment position="start">
                    <SearchIcon />
                  </InputAdornment>
                  {params.InputProps.startAdornment}
                </>
              ),
            }}
          />
        );
      }}
      renderOption={(props: any, option: any) => {
        if (option.username !== undefined) {
          return (
            <List
              {...props}
              style={{
                maxWidth: '100%',
                height: 'max-content',
                padding: '0.1rem 0',
              }}
            >
              <Link
                href={`/profile/${option.username}`}
              >
                <ListItem
                  style={{
                    width: '100%',
                    textOverflow: 'ellipsis',
                    overflow: 'hidden',
                    height: '50px',
                    padding: '0',
                  }}
                  key={option.id}
                >
                  <ListItemButton
                    style={{
                      width: '100%',
                      textOverflow: 'ellipsis',
                      overflow: 'hidden',
                    }}
                  >
                    <Badge
                      color="primary"
                      badgeContent="User"
                      overlap="circular"
                      anchorOrigin={{
                        vertical: 'top',
                        horizontal: 'left',
                      }}
                    >
                      <Avatar src={option.headImgUrl} sx={{ mr: 1.5 }} />
                    </Badge>
                    <span
                      style={{
                        textDecoration: 'none',
                        color: 'black',
                        whiteSpace: 'nowrap',
                        textOverflow: 'ellipsis',
                        overflow: 'hidden',
                        maxWidth: '100%',
                      }}
                    >
                      {`${option.username}`}
                    </span>
                  </ListItemButton>
                </ListItem>
              </Link>
            </List>
          );
        }
        if (option.title !== undefined) {
          return (
            <List
              {...props}
              style={{
                maxWidth: '100%',
                height: 'max-content',
                padding: '0.1rem 0',
              }}
            >
              <Link
                href={`/post/${option.id}`}

              >
                <ListItem
                  style={{
                    width: '100%',
                    textOverflow: 'ellipsis',
                    overflow: 'hidden',
                    height: '50px',
                    padding: '0',
                  }}
                  key={option.id}
                >
                  <ListItemButton
                    style={{
                      width: '100%',
                      textOverflow: 'ellipsis',
                      overflow: 'hidden',
                    }}
                  >
                    <Badge
                      color="secondary"
                      badgeContent="Post"
                      overlap="circular"
                      anchorOrigin={{
                        vertical: 'top',
                        horizontal: 'left',
                      }}
                    >
                      <Avatar src={option.headImgUrl} sx={{ mr: 1.5 }} />
                    </Badge>
                    <span
                      style={{
                        textDecoration: 'none',
                        color: 'black',
                        whiteSpace: 'nowrap',
                        textOverflow: 'ellipsis',
                        overflow: 'hidden',
                        maxWidth: '100%',
                      }}
                    >
                      {`${option.title}`}
                    </span>
                  </ListItemButton>
                </ListItem>
              </Link>
            </List>
          );
        }
        return null;
      }}
    />
  );
};

export default SearchBar;
