import { ThemeProvider } from '@/components/theme-provider';
import { Dashboard } from '@/components/Dashboard';
import { Toaster } from 'sonner';
import "@/App.css";

function App() {
  return (
    <ThemeProvider defaultTheme="light" storageKey="lemme-do-it-theme">
      <Dashboard />
      <Toaster position="top-right" richColors />
    </ThemeProvider>
  );
}

export default App;