import { useState, useEffect } from 'react';
import { proto_to_typescript, proto_to_swift, proto_to_kotlin, proto_to_dart } from './wasm/xcutils_wasm.js';
import Prism from 'prismjs';
import 'prismjs/themes/prism-tomorrow.css';
import 'prismjs/components/prism-typescript';
import 'prismjs/components/prism-swift';
import 'prismjs/components/prism-kotlin';
import 'prismjs/components/prism-dart';

// 导入Material-UI组件
import { ThemeProvider, createTheme } from '@mui/material/styles';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import MenuItem from '@mui/material/MenuItem';
import Select from '@mui/material/Select';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Paper from '@mui/material/Paper';
import IconButton from '@mui/material/IconButton';
import Tooltip from '@mui/material/Tooltip';
import Snackbar from '@mui/material/Snackbar';
import Alert from '@mui/material/Alert';
import ContentCopyIcon from '@mui/icons-material/ContentCopy';

// 创建主题
const theme = createTheme();

export default function App() {
  const [protoContent, setProtoContent] = useState('');
  const [output, setOutput] = useState('');
  const [outputType, setOutputType] = useState('swift');
  const [copySuccess, setCopySuccess] = useState(false);
  const [copyError, setCopyError] = useState(false);

  useEffect(() => {
    Prism.highlightAll();
  }, [output]);

  const splitAndConvertProto = (content, converter) => {
    // 按message和enum拆分proto内容
    const parts = content.split(/(?=message\s+\w+|enum\s+\w+)/g);
    let result = '';

    parts.forEach(part => {
      if (part.trim()) {
        try {
          const converted = converter(part);
          result += converted + '\n\n';
        } catch (e) {
          result += `// Error converting part:\n${part}\n// Error: ${e}\n\n`;
        }
      }
    });

    return result || '// No valid proto message or enum found';
  };

  const handleConvert = () => {
    try {
      let result = '';
      switch (outputType) {
        case 'typescript':
          result = splitAndConvertProto(protoContent, proto_to_typescript);
          break;
        case 'swift':
          result = splitAndConvertProto(protoContent, proto_to_swift);
          break;
        case 'kotlin':
          result = splitAndConvertProto(protoContent, proto_to_kotlin);
          break;
        case 'dart':
          result = splitAndConvertProto(protoContent, proto_to_dart);
          break;
      }
      setOutput(result);
    } catch (e) {
      setOutput(`Error: ${e}`);
    }
  };

  const handleCopyToClipboard = async () => {
    if (!output.trim()) {
      setCopyError(true);
      return;
    }

    try {
      await navigator.clipboard.writeText(output);
      setCopySuccess(true);
    } catch (err) {
      // 如果现代API失败，尝试使用传统方法
      try {
        const textArea = document.createElement('textarea');
        textArea.value = output;
        document.body.appendChild(textArea);
        textArea.select();
        document.execCommand('copy');
        document.body.removeChild(textArea);
        setCopySuccess(true);
      } catch (fallbackErr) {
        setCopyError(true);
      }
    }
  };

  const handleCloseSnackbar = () => {
    setCopySuccess(false);
    setCopyError(false);
  };

  return (
    <ThemeProvider theme={theme}>
      <Box sx={{
        maxWidth: 1200,
        margin: '0 auto',
        padding: 3
      }}>
        <Typography variant="h3" gutterBottom>
          XCUtils Proto Converter
        </Typography>

        <Box sx={{
          display: 'flex',
          alignItems: 'center',
          gap: 2,
          mb: 3
        }}>
      
          <Select
            value={outputType}
            onChange={(e) => setOutputType(e.target.value)}
            sx={{ minWidth: 200 }}
          >
            <MenuItem value="swift">Swift</MenuItem>
            <MenuItem value="typescript">TypeScript</MenuItem>
            <MenuItem value="kotlin">Kotlin</MenuItem>
            <MenuItem value="dart">Dart</MenuItem>
          </Select>

          <Button
            variant="contained"
            onClick={handleConvert}
          >
            Convert
          </Button>
        </Box>

        <Box sx={{ mb: 3 }}>
          <TextField
            multiline
            fullWidth
            rows={8}
            value={protoContent}
            onChange={(e) => setProtoContent(e.target.value)}
            placeholder="Enter Proto content here..."
            variant="outlined"
          />
        </Box>

        <Box>
          <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', mb: 1 }}>
            <Typography variant="subtitle1">
              Output:
            </Typography>
            {output.trim() && (
              <Tooltip title="复制到剪贴板">
                <IconButton 
                  onClick={handleCopyToClipboard}
                  color="primary"
                  size="small"
                >
                  <ContentCopyIcon />
                </IconButton>
              </Tooltip>
            )}
          </Box>
          <Paper elevation={3} sx={{ position: 'relative' }}>
            <pre key={`${outputType}-${output.length}`} className={`language-${outputType === 'kotlin' ? 'kotlin' : outputType}`}>
              <code className={`language-${outputType === 'kotlin' ? 'kotlin' : outputType}`}>{output}</code>
            </pre>
          </Paper>
        </Box>

        {/* 复制成功/失败的通知 */}
        <Snackbar
          open={copySuccess}
          autoHideDuration={3000}
          onClose={handleCloseSnackbar}
          anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
        >
          <Alert onClose={handleCloseSnackbar} severity="success" sx={{ width: '100%' }}>
            代码已成功复制到剪贴板！
          </Alert>
        </Snackbar>

        <Snackbar
          open={copyError}
          autoHideDuration={3000}
          onClose={handleCloseSnackbar}
          anchorOrigin={{ vertical: 'bottom', horizontal: 'center' }}
        >
          <Alert onClose={handleCloseSnackbar} severity="error" sx={{ width: '100%' }}>
            复制失败，请手动选择并复制代码。
          </Alert>
        </Snackbar>
      </Box>
    </ThemeProvider>
  );
}