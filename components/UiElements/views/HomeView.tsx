import { StyleSheet } from "react-native";
import NavButton from "@/components/UiElements/buttons/NavButton";
import { Avatar, useTheme } from "react-native-paper";
import PageContainer from "../container/PageContainer";
import NeonText from "../textModifier/NeonText";

export default function HomeView() {
  const theme = useTheme();

  return (
    <PageContainer>
      <NeonText txt="LeShaker" size="displayLarge" />
      <Avatar.Image
        size={300}
        source={require("@/assets/images/logo/logo.png")}
      />
      <NavButton path="/(modes)" pathName="JOUER" />
    </PageContainer>
  );
}

const styles = StyleSheet.create({
  PageContainer: {
    flex: 1,
    justifyContent: "space-between",
    alignItems: "center",
  },
});
