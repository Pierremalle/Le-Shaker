import { PaperProvider, MD3DarkTheme, MD3LightTheme } from "react-native-paper";
import { useColorScheme } from "react-native";
import { StatusBar } from "react-native";

type ThemeContextProps = {
  children: React.ReactNode;
};

export default function ThemeProvider({ children }: ThemeContextProps) {
  const colorScheme = useColorScheme();

  const paperTheme =
    colorScheme === "dark"
      ? {
          ...MD3DarkTheme,
          colors: {
            primary: "rgb(211, 187, 255)",
            onPrimary: "rgb(63, 0, 141)",
            primaryContainer: "rgb(87, 39, 167)",
            onPrimaryContainer: "rgb(235, 221, 255)",
            secondary: "rgb(205, 194, 219)",
            onSecondary: "rgb(52, 45, 64)",
            secondaryContainer: "rgb(75, 67, 88)",
            onSecondaryContainer: "rgb(233, 222, 248)",
            tertiary: "rgb(240, 183, 197)",
            onTertiary: "rgb(74, 37, 48)",
            tertiaryContainer: "rgb(100, 59, 70)",
            onTertiaryContainer: "rgb(255, 217, 225)",
            error: "rgb(255, 180, 171)",
            onError: "rgb(105, 0, 5)",
            errorContainer: "rgb(147, 0, 10)",
            onErrorContainer: "rgb(255, 180, 171)",
            background: "rgb(29, 27, 30)",
            onBackground: "rgb(230, 225, 230)",
            surface: "rgb(29, 27, 30)",
            onSurface: "rgb(230, 225, 230)",
            surfaceVariant: "rgb(73, 69, 78)",
            onSurfaceVariant: "rgb(203, 196, 207)",
            outline: "rgb(148, 143, 153)",
            outlineVariant: "rgb(73, 69, 78)",
            shadow: "rgb(0, 0, 0)",
            scrim: "rgb(0, 0, 0)",
            inverseSurface: "rgb(230, 225, 230)",
            inverseOnSurface: "rgb(50, 48, 51)",
            inversePrimary: "rgb(111, 67, 192)",
            elevation: {
              level0: "transparent",
              level1: "rgb(38, 35, 41)",
              level2: "rgb(44, 40, 48)",
              level3: "rgb(49, 45, 55)",
              level4: "rgb(51, 46, 57)",
              level5: "rgb(55, 49, 62)",
            },
            surfaceDisabled: "rgba(230, 225, 230, 0.12)",
            onSurfaceDisabled: "rgba(230, 225, 230, 0.38)",
            backdrop: "rgba(50, 47, 55, 0.4)",
            yellow: "rgb(205, 204, 74)",
            onYellow: "rgb(50, 50, 0)",
            yellowContainer: "rgb(74, 73, 0)",
            onYellowContainer: "rgb(233, 233, 99)",
          },
        }
      : {
          ...MD3LightTheme,
          colors: {
            primary: "rgb(111, 67, 192)",
            onPrimary: "rgb(255, 255, 255)",
            primaryContainer: "rgb(235, 221, 255)",
            onPrimaryContainer: "rgb(37, 0, 89)",
            secondary: "rgb(99, 91, 112)",
            onSecondary: "rgb(255, 255, 255)",
            secondaryContainer: "rgb(233, 222, 248)",
            onSecondaryContainer: "rgb(31, 24, 43)",
            tertiary: "rgb(126, 82, 94)",
            onTertiary: "rgb(255, 255, 255)",
            tertiaryContainer: "rgb(255, 217, 225)",
            onTertiaryContainer: "rgb(49, 16, 27)",
            error: "rgb(186, 26, 26)",
            onError: "rgb(255, 255, 255)",
            errorContainer: "rgb(255, 218, 214)",
            onErrorContainer: "rgb(65, 0, 2)",
            background: "rgb(255, 251, 255)",
            onBackground: "rgb(29, 27, 30)",
            surface: "rgb(255, 251, 255)",
            onSurface: "rgb(29, 27, 30)",
            surfaceVariant: "rgb(231, 224, 235)",
            onSurfaceVariant: "rgb(73, 69, 78)",
            outline: "rgb(122, 117, 127)",
            outlineVariant: "rgb(203, 196, 207)",
            shadow: "rgb(0, 0, 0)",
            scrim: "rgb(0, 0, 0)",
            inverseSurface: "rgb(50, 48, 51)",
            inverseOnSurface: "rgb(245, 239, 244)",
            inversePrimary: "rgb(211, 187, 255)",
            elevation: {
              level0: "transparent",
              level1: "rgb(248, 242, 252)",
              level2: "rgb(244, 236, 250)",
              level3: "rgb(239, 231, 248)",
              level4: "rgb(238, 229, 247)",
              level5: "rgb(235, 225, 246)",
            },
            surfaceDisabled: "rgba(29, 27, 30, 0.12)",
            onSurfaceDisabled: "rgba(29, 27, 30, 0.38)",
            backdrop: "rgba(50, 47, 55, 0.4)",
            yellow: "rgb(98, 98, 0)",
            onYellow: "rgb(255, 255, 255)",
            yellowContainer: "rgb(233, 233, 99)",
            onYellowContainer: "rgb(29, 29, 0)",
          },
        };
  return (
    <PaperProvider theme={paperTheme}>
      <StatusBar hidden />
      {children}
    </PaperProvider>
  );
}
