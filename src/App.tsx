import { useEffect, useState } from "react";
import { isFirstRun, checkPermissions } from "./lib/commands";
import { useI18n } from "./lib/i18n";
import { SetupWizard } from "./components/SetupWizard";
import { PermissionSetup } from "./components/PermissionSetup";
import { Settings } from "./components/Settings";
import "./styles/global.css";

function App() {
  const { t } = useI18n();
  const [firstRun, setFirstRun] = useState<boolean | null>(null);
  const [permissionsOk, setPermissionsOk] = useState<boolean | null>(null);

  useEffect(() => {
    isFirstRun().then(setFirstRun);
    checkPermissions().then((s) => {
      setPermissionsOk(s.microphone && s.accessibility);
    });
  }, []);

  if (firstRun === null || permissionsOk === null) {
    return <div className="loading">{t("app.loading")}</div>;
  }

  if (firstRun) {
    return <SetupWizard onComplete={() => setFirstRun(false)} />;
  }

  if (!permissionsOk) {
    return <PermissionSetup onAllGranted={() => setPermissionsOk(true)} />;
  }

  return <Settings />;
}

export default App;
