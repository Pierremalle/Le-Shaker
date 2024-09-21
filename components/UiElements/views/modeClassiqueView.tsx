import { Text, useTheme } from "react-native-paper";
import PageContainer from "../container/PageContainer";
import PlayerLister from "../specials/PlayersLister";
import { View, StyleSheet } from "react-native";

export default function ModeClassiqueView() {
  const theme = useTheme();

  return (
    <PageContainer>
      <View style={styles.TexContainer}>
        <Text
          variant="headlineLarge"
          theme={{ colors: { primary: theme.colors.onSurface } }}
        >
          Ajoutez vos joueurs
        </Text>
      </View>
      <PlayerLister />
    </PageContainer>
  );
}

const styles = StyleSheet.create({
  TexContainer: {
    padding: 30,
  },
});
