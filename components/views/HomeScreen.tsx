import React from "react";
import { Button, View } from "react-native";
import { NativeStackNavigationProp } from "@react-navigation/native-stack";

type RootStackParamList = {
  Home: undefined;
  "Second Page": { name: string };
};

type HomeScreenNavigationProp = NativeStackNavigationProp<
  RootStackParamList,
  "Home"
>;

type Props = {
  navigation: HomeScreenNavigationProp;
};

export default function HomeScreen({ navigation }: Props) {
  return (
    <View>
      <Button
        title="Go to Jane's profile"
        onPress={() => navigation.navigate("Second Page", { name: "Jane" })}
      />
    </View>
  );
}
