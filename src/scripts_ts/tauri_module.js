//This file is needed because typescript uses different scope for files with import declarations

import { open } from '@tauri-apps/api/dialog';

window.tauri_file_dialog_open = open;