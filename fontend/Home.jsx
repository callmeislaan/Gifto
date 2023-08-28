const setShowNewHackathonModal = props.setShowNewHackathonModal;

const convertTimestampToDatetime = (timestampNanoseconds) => {
  const milliseconds = Math.floor(timestampNanoseconds / 1000000);
  return new Date(milliseconds);
};

function convertDatetimeToTimestamp(date) {
  const milliseconds = date.getTime();
  return milliseconds * 1000000;
}

const currentTime = convertDatetimeToTimestamp(new Date());

State.init({
  hackathon_id: null,
  showHackathon: false,
  hackathonBaseData: null,
  isMember: props.isMember || false,
  hackathons: [],
});

const setShowHackathon = (value) => {
  State.update({
    showHackathon: value,
  });
};

const setHackathonId = (id) => {
  State.update({
    hackathon_id: id,
  });
};

const setHackathonBaseData = (data) => {
  State.update({
    hackathonBaseData: data,
  });
};

function convertStringToFloat(inputString) {
  return (parseFloat(inputString) / Math.pow(10, 24)).toFixed(2);
}

function sliceString(longString, length) {
  if (longString.length <= length) {
    return longString;
  } else {
    return longString.slice(0, length) + "...";
  }
}

const Home = styled.div`
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    flex-direction: column;
    justify-content: start;
    `;

const HomeBar = styled.div`
    min-width: 70%;
    height: 50px;
    display: flex;
    align-items: center;
    flex-direction: row;
    justify-content: space-between;
    margin: 10px;
    padding: 10px;
    `;

const HackathonSmallWidget = styled.div`
    max-width: 70%;
    height: fit-content;
    display: flex;
    align-items: start;
    flex-direction: row;
    margin: 10px;
    padding: 10px;
    border: solid thin black;
`;

const FlexBetween = styled.div`
    display: flex;
    flex-direction: row;
    justify-content: space-between;
`;

const Box = styled.div`
    display: flex;
    width: 100%;
    height: 100%;
`;

const TagsBox = styled.div`
    border: solid thin blue;
    padding: 5px;
    border-radius: 15px;
    margin: 2px;
    margin-left: 10px;
    height: fit-content;
`;

State.update({
  hackathons: Near.view(
    "project.duyan.testnet",
    "get_all_hackathons",
    "{}"
  ).reverse(),
});

return (
  <Home>
    {state.showHackathon ? (
      <div style={{ maxWidth: "70%" }}>
        <Widget
          src={`duyan.testnet/widget/HackathonWidget`}
          props={{
            setShowRegisterModal: props.setShowRegisterModal,
            isMember: state.isMember,
            convertTimestampToDatetime: convertTimestampToDatetime,
            convertDatetimeToTimestamp: convertDatetimeToTimestamp,
            hackathonBaseData: state.hackathonBaseData,
            hackathon_id: state.hackathon_id,
            setShowHackathon: setShowHackathon,
          }}
        />
      </div>
    ) : (
      <>
        <HomeBar>
          <h1>Hackathons</h1>
          <button
            onClick={() => {
              state.isMember
                ? setShowNewHackathonModal(true)
                : props.setShowRegisterModal(true);
            }}
          >
            create new hackathon
          </button>
        </HomeBar>
        <>
          {state.hackathons.length != 0 ? (
            state.hackathons.map((h, i) => (
              <HackathonSmallWidget
                onClick={() => {
                  setShowHackathon(true);
                  setHackathonId(h.hackathon.id);
                  setHackathonBaseData(h);
                }}
              >
                <img
                  src={`https://ipfs.near.social/ipfs/${h.hackathon.image}`}
                  style={{
                    width: 250,
                    height: 250,
                    marginRight: 15,
                  }}
                />
                <Box
                  style={{
                    flexDirection: "column",
                    justifyContent: "start",
                  }}
                >
                  <FlexBetween>
                    <FlexBetween>
                      <h2>{h.hackathon.name}</h2>
                    </FlexBetween>

                    <div
                      style={{
                        backgroundColor: "#009ACD",
                        display: "flex",
                        justifyContent: "center",
                        alignItems: "center",
                        paddingLeft: 10,
                        paddingRight: 10,
                        borderRadius: 20,
                      }}
                    >
                      {currentTime < h.hackathon.start
                        ? "upcoming"
                        : currentTime < h.hackathon.end
                        ? "ongoing"
                        : "goned"}
                    </div>
                  </FlexBetween>
                  <p>
                    {h.hackathon.owner == context.accountId
                      ? "Owned by you"
                      : `@${h.hackathon.owner}`}
                  </p>

                  <Box>
                    <p>{sliceString(h.hackathon.description, 150)}</p>
                  </Box>
                  <Box>
                    <h5>
                      {convertTimestampToDatetime(
                        h.hackathon.start
                      ).toDateString()}
                      -
                      {convertTimestampToDatetime(
                        h.hackathon.end
                      ).toDateString()}
                    </h5>
                  </Box>
                  <Box
                    style={{
                      justifyContent: "space-between",
                    }}
                  >
                    <p>{convertStringToFloat(h.total_prize)}Near in prize</p>
                    <p>{h.hackathon.participants_list.length} participants</p>
                  </Box>
                  <Box>
                    {h.hackathon.tags.map((t) => (
                      <TagsBox> {t}</TagsBox>
                    ))}
                  </Box>
                </Box>
              </HackathonSmallWidget>
            ))
          ) : (
            <h1>No Hackathon Created</h1>
          )}
        </>
      </>
    )}
  </Home>
);
