export type Resolution = {
  width: number;
  height: number;
};

export type InputResolutions = {
  input_1: Resolution;
  input_2: Resolution;
  input_3: Resolution;
  input_4: Resolution;
  input_5: Resolution;
  input_6: Resolution;
};

export const AVAILABLE_RESOLUTIONS = {
  Resoultion1080x1920: {
    width: 1080,
    height: 1920,
  },
  Resoultion1920x1080: {
    width: 1920,
    height: 1080,
  },
  Resoultion854x480: {
    width: 854,
    height: 480,
  },
  Resoultion480x854: {
    width: 480,
    height: 854,
  },
  Resoultion1440x1080: {
    width: 1440,
    height: 1080,
  },
  Resoultion1080x1440: {
    width: 1080,
    height: 1440,
  },
} as const;

export enum ResolutionName {
  Resoultion1080x1920 = 'Resoultion1080x1920',
  Resoultion1920x1080 = 'Resoultion1920x1080',
  Resoultion854x480 = 'Resoultion854x480',
  Resoultion480x854 = 'Resoultion480x854',
  Resoultion1440x1080 = 'Resoultion1440x1080',
  Resoultion1080x1440 = 'Resoultion1080x1440',
}

export type InputResolutionNames = {
  input_1: ResolutionName;
  input_2: ResolutionName;
  input_3: ResolutionName;
  input_4: ResolutionName;
  input_5: ResolutionName;
  input_6: ResolutionName;
};

export function inputResolutionNamesToResolutions(
  inputResolutionNames: InputResolutionNames
): InputResolutions {
  return {
    input_1: nameToResolution(inputResolutionNames.input_1),
    input_2: nameToResolution(inputResolutionNames.input_2),
    input_3: nameToResolution(inputResolutionNames.input_3),
    input_4: nameToResolution(inputResolutionNames.input_4),
    input_5: nameToResolution(inputResolutionNames.input_5),
    input_6: nameToResolution(inputResolutionNames.input_6),
  };
}

function nameToResolution(name: ResolutionName): Resolution {
  switch (name) {
    case ResolutionName.Resoultion1920x1080:
      return AVAILABLE_RESOLUTIONS.Resoultion1920x1080;
    case ResolutionName.Resoultion1080x1920:
      return AVAILABLE_RESOLUTIONS.Resoultion1080x1920;
    case ResolutionName.Resoultion1440x1080:
      return AVAILABLE_RESOLUTIONS.Resoultion1440x1080;
    case ResolutionName.Resoultion1080x1440:
      return AVAILABLE_RESOLUTIONS.Resoultion1080x1440;
    case ResolutionName.Resoultion854x480:
      return AVAILABLE_RESOLUTIONS.Resoultion854x480;
    case ResolutionName.Resoultion480x854:
      return AVAILABLE_RESOLUTIONS.Resoultion480x854;
  }
}
