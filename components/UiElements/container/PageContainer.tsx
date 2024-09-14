import { SafeAreaView, StyleSheet } from "react-native";
import { useTheme } from "react-native-paper";

type ThemeContextProps = {
  children: React.ReactNode;
};

export default function PageContainer({ children }: ThemeContextProps) {
  const theme = useTheme();
  return (
    <SafeAreaView
      style={[styles.container, { backgroundColor: theme.colors.surface }]}
    >
      {children}
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: "space-evenly",
    alignItems: "center",
    height: "100%", // Add this
    width: "100%", // Add this
  },
});
