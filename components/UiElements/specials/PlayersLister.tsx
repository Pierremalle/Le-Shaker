import { StyleSheet, View } from "react-native";
import { useState } from "react";
import { useTheme } from "react-native-paper";
import SinglePlayerCase from "./SinglePlayerCase";

export default function PlayerLister() {
  const theme = useTheme();

  const [joueurs, setJoueurs] = useState([{ playerName: "empty", id: -1 }]);

  return (
    <View style={styles.ListeContainer}>
      {joueurs.map((joueur) => (
        <SinglePlayerCase
          key={joueur.id}
          playerName={joueur.playerName}
        ></SinglePlayerCase>
      ))}
    </View>
  );
}

const styles = StyleSheet.create({
  ListeContainer: {
    flex: 1,
    justifyContent: "space-between",
    alignItems: "center",
    width: "80%",
    height: "50%",
    borderColor: "red",
    borderWidth: 3,
  },
});
