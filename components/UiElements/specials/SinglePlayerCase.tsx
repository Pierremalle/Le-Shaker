import { Text } from "react-native-paper";

interface SinglePlayserCaseInterface {
  playerName: string;
}

export default function SinglePlayerCase({
  playerName,
}: SinglePlayserCaseInterface) {
  return (
    <>
      {playerName === "empty" ? (
        <Text>Empty Player</Text>
      ) : (
        <Text>{playerName}</Text>
      )}
    </>
  );
}
