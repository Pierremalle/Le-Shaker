import { View, Text, StyleSheet } from "react-native";
import NavButton from "@/components/UiElements/NavButton";

export default function HomeScreen() {
  return (
    <View style={styles.container}>
      <Text>Home</Text>
      <NavButton path="/(modes)" pathName="JOUER" />
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "center",
    alignItems: "center",
  },
});
