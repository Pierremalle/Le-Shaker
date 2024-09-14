import { Text, List } from "react-native-paper";
import PageContainer from "@/components/UiElements/container/PageContainer";
import { modesInfos } from "@/assets/data/modesInfos";

export default function ModeList() {
  return (
    <PageContainer>
      {modesInfos.map((mode) => (
        <List.Item
          title={mode.title}
          description={mode.description}
          left={(props) => <List.Icon {...props} icon="folder" />}
        />
      ))}
    </PageContainer>
  );
}
