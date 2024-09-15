import { Avatar, List, useTheme } from "react-native-paper";
import PageContainer from "@/components/UiElements/container/PageContainer";
import { modesInfos } from "@/assets/data/modesInfos";
import { StyleSheet } from "react-native";
import { View } from "react-native";
import { router } from "expo-router";

export default function ModeList() {
  const theme = useTheme();

  return (
    <PageContainer>
      <View style={styles.container}>
        {modesInfos.map((mode) => (
          <List.Item
            style={[styles.listItem, { borderColor: theme.colors.onSurface }]}
            key={mode.key}
            title={mode.title}
            description={mode.description}
            left={() => (
              <Avatar.Image
                size={60}
                source={require("@/assets/images/logo/logo.png")}
              />
            )}
            onPress={() => {
              router.push(mode.path);
            }}
          />
        ))}
      </View>
    </PageContainer>
  );
}

const styles = StyleSheet.create({
  listItem: {
    borderRadius: 30,
    borderWidth: 3,
    padding: 10,
    margin: 5,
  },
  container: {
    flex: 1,
    height: "100%",
    width: "100%",
    margin: 20,
  },
});
