import { UmiPlugin } from '@metaplex-foundation/umi';
import { createBglDialogManagerProgram } from './generated';

export const bglDialogManager = (): UmiPlugin => ({
  install(umi) {
    umi.programs.add(createBglDialogManagerProgram(), false);
  },
});
