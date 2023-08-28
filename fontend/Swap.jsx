// MIT License: https://github.com/DV-Lab/zswap-frontend-bos/blob/main/LICENSE

/* FOR STYLING */
const App = styled.div`
  position: relative;
  width: 100%;
  min-height: 100vh;
  padding: 8px;
`;

const MainLayout = styled.div`
  width: 100%;
  height: calc(100% - 8px);
  border-radius: 20px;
  box-sizing: border-box;
`;

const Container = styled.div`
  width: 100%;
  height: 100%;
  margin-top: 20px;
  padding: 20px 12px;
  display: flex;
  flex-direction: column;
  align-items: center;
`;

const OverflowContainer = styled.div`
  width: 100%;
  padding: 4px 8px;
  overflow-y: scroll;
  display: flex;
  flex-direction: column;
  align-items: center;
  &::-webkit-scrollbar-track {
    -webkit-box-shadow: inset 0 0 6px rgba(255, 255, 255, 0.1);
    background-color: transparent;
    border-radius: 10px;
    margin-top: 0px;
    margin-left: 4px;
  }
  &::-webkit-scrollbar {
    width: 8px;
    background-color: transparent;
  }
  &::-webkit-scrollbar-thumb {
    background-color: #3cd89d;
    border: 1px solid rgba(252, 254, 231, 1);
  }
`;

// Config for ZSwap app
function getConfig(network) {
  switch (network) {
    case "mainnet":
      return {
        ownerId: "zswapprotocol.near",
        contractId: "zswap-protocol.near",
        nodeUrl: "https://rpc.mainnet.near.org",
        appUrl: "https://app.zswapprotocol.org",
      };
    case "testnet":
      return {
        ownerId: "zswap.testnet",
        contractId: "zswap-protocol.testnet",
        nodeUrl: "https://rpc.testnet.near.org",
        appUrl: "https://testnet.zswapprotocol.org",
      };
    default:
      throw Error(`Unconfigured environment '${network}'.`);
  }
}
const config = getConfig(context.networkId);

const RouterRecords = {
  swap: {
    name: "swap",
    title: "Swap",
    description: "Swap tokens",
    path: "widget/ZSwap.Page.Swap.MainSwap",
  },
  pools: {
    name: "pools",
    title: "Pools",
    description: "View pools",
    path: "widget/ZSwap.Page.Pools.Pools",
  },
  tokens: {
    name: "tokens",
    title: "Tokens",
    description: "View tokens",
    path: "widget/ZSwap.Page.Tokens.Tokens",
  },
};

State.init({
  page: RouterRecords.swap.name,
  unstakeInfo: {},
});

const updatePage = (pageName) => {
  State.update({ page: pageName });
};

const SwapView = () => {
  return (
    <Container>
      <Widget
        src={`${config.ownerId}/widget/ZSwap.Element.TitleAndDescription`}
        props={{
          title: RouterRecords.swap.title,
          description: RouterRecords.swap.description,
        }}
      />
      <Widget
        src={`${config.ownerId}/${RouterRecords.swap.path}`}
        props={{ config }}
      />
    </Container>
  );
};

const PoolView = () => {
  return (
    <Container>
      <Widget
        src={`${config.ownerId}/widget/ZSwap.Element.TitleAndDescription`}
        props={{
          title: RouterRecords.pools.title,
          description: RouterRecords.pools.description,
          config: config,
        }}
      />
      <Widget
        src={`${config.ownerId}/${RouterRecords.pools.path}`}
        props={{ config }}
      />
    </Container>
  );
};

const TokenView = () => {
  return (
    <Container>
      <Widget
        src={`${config.ownerId}/widget/ZSwap.Element.TitleAndDescription`}
        props={{
          title: RouterRecords.tokens.title,
          description: RouterRecords.tokens.description,
        }}
      />
      <div
        style={{
          marginTop: "20px",
          width: "100%",
          marginTop: "20px",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
        }}
      >
        <Widget src={`${config.ownerId}/${RouterRecords.tokens.path}`} />
      </div>
    </Container>
  );
};

function getBody() {
  switch (state.page) {
    case RouterRecords.swap.name:
      return <SwapView />;
    case RouterRecords.pools.name:
      return <PoolView />;
    case RouterRecords.tokens.name:
      return <TokenView />;
  }
}

return (
  <App>
    <MainLayout>
      <Widget
        src={`${config.ownerId}/widget/ZSwap.Layout.Navigation`}
        props={{
          updatePage,
          page: state.page,
        }}
      />
      {getBody()}
    </MainLayout>
  </App>
);
