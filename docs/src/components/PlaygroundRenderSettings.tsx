import SubmitButton from './SubmitButton';

function PlaygroundRenderSettings({
  onSubmit,
  readyToSubmit,
}: {
  onSubmit: () => Promise<void>;
  readyToSubmit: boolean;
}) {
  return (
    <div style={{ margin: '10px' }}>
      <div className="row">
        <div className="col">Settings:</div>
        <div className="col">
          <select>
            <option value="someOption">Some option</option>
            <option value="otherOption">Other option</option>
          </select>
        </div>
        <div className="col">
          <select>
            <option value="someOption">Some option</option>
            <option value="otherOption">Other option</option>
          </select>
        </div>
        <div className="col">
          <select>
            <option value="someOption">Some option</option>
            <option value="otherOption">Other option</option>
          </select>
        </div>
        <div className="col">
          <SubmitButton onSubmit={onSubmit} readyToSubmit={readyToSubmit} />
        </div>
      </div>
    </div>
  );
}

export default PlaygroundRenderSettings;
