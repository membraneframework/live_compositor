import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';

import styles from './playground.module.css';
import { useState } from 'react';
import PlaygroundRenderSettings from '../components/PlaygroundRenderSettings';
import PlaygroundPreview from '../components/PlaygroundPreview';
import PlaygroundCodeEditor from '../components/PlaygroundCodeEditor';
import toast, { Toaster } from 'react-hot-toast';
import { renderImage } from '../api';
import 'react-tooltip/dist/react-tooltip.css';

const INITIAL_SCENE = {
  type: 'view',
};

function Homepage() {
  const [scene, setScene] = useState<object | Error>(INITIAL_SCENE);

  const [responseData, setResponseData] = useState({
    imageUrl: '',
    errorMessage: '',
  });

  const setErrorMessage = message => {
    setResponseData(prevResponseData => ({ ...prevResponseData, errorMessage: message }));
  };

  const handleSubmit = async (): Promise<void> => {
    try {
      if (scene instanceof Error) {
        throw new Error(`${scene.name};\n${scene.message}`);
      }
      const blob = await renderImage({ scene });
      const imageObjectURL = URL.createObjectURL(blob);

      setResponseData({ imageUrl: imageObjectURL, errorMessage: '' });
    } catch (error: any) {
      setErrorMessage(error.message);
      toast.error(`${error.message}`);
    }
  };

  return (
    <div className={styles.page}>
      <div className={styles.leftSide}>
        <div className={styles.codeEditorBox}>
          <PlaygroundCodeEditor onChange={setScene} initialCodeEditorContent={INITIAL_SCENE} />
        </div>
      </div>
      <div className={styles.rightSide}>
        <div className={styles.preview}>
          <PlaygroundPreview {...responseData} />
        </div>
        <div className={styles.settingsBox}>
          <PlaygroundRenderSettings
            onSubmit={handleSubmit}
            readyToSubmit={!(scene instanceof Error)}
          />
        </div>
      </div>
    </div>
  );
}

export default function Home() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout title={siteConfig.title}>
      <Toaster />
      <Homepage />
    </Layout>
  );
}
