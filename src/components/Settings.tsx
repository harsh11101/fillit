import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Label } from '@/components/ui/label';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';
import { toast } from 'sonner';
import { Clock, Info } from 'lucide-react';
import { SnippetSettings } from '@/types';

export function Settings() {
  const [timeDelay, setTimeDelay] = useState(200);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadSettings();
  }, []);

  const loadSettings = async () => {
    try {
      setLoading(true);
      const delay = await invoke<SnippetSettings>('get_snippets_settings');
      console.log('Loaded time delay:', delay);
      setTimeDelay(delay.time_delay_ms);
    } catch (error) {
      toast.error('Failed to load settings');
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      if (timeDelay < 0) {
        toast.error('Time delay cannot be negative');
        return;
      }
      await invoke('update_snippet_settings', { time_delay_ms: timeDelay });
      toast.success(`Time delay set to ${timeDelay}ms`);
    } catch (error) {
      console.error(error);
      toast.error(error as string);
    }
  };

  const handleReset = async () => {
    try {
      await invoke('update_snippet_settings', { time_delay_ms: 200 });
      setTimeDelay(200);
      toast.success('Settings reset to default (200ms)');
    } catch (error) {
      console.error(error);
      toast.error('Failed to reset settings');
    }
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center py-12">
        <p className="text-muted-foreground">Loading settings...</p>
      </div>
    );
  }

  return (
    <div className="max-w-2xl mx-auto space-y-6">
      <Card>
        <CardHeader>
          <div className="flex items-center gap-2">
            <Clock className="w-5 h-5" />
            <CardTitle>Snippet Expansion Settings</CardTitle>
          </div>
          <CardDescription>
            Configure the time delay for automatic snippet expansion
          </CardDescription>
        </CardHeader>
        <CardContent className="space-y-6">
          {/* Info Card */}
          <div className="p-4 rounded-lg bg-blue-50 dark:bg-blue-950 border border-blue-200 dark:border-blue-800">
            <div className="flex gap-2">
              <Info className="w-4 h-4 text-blue-600 dark:text-blue-400 mt-0.5 flex-shrink-0" />
              <div className="text-sm text-blue-900 dark:text-blue-200">
                <strong>How it works:</strong> After you finish typing a trigger, the app waits for the configured 
                delay before automatically expanding it to your snippet content. Set the delay to 0ms for instant expansion.
              </div>
            </div>
          </div>

          {/* Time Delay Settings */}
          <div className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="time-delay" className="text-base">Time Delay (milliseconds)</Label>
              <div className="flex gap-3">
                <Input
                  id="time-delay"
                  type="number"
                  min="0"
                  step="50"
                  value={timeDelay}
                  onChange={(e) => {
                    const value = parseInt(e.target.value);
                    if (!isNaN(value) && value >= 0) {
                      setTimeDelay(value);
                    }
                  }}
                  className="text-lg font-semibold"
                />
                <div className="flex items-center px-3 text-muted-foreground bg-muted rounded-md">
                  ms
                </div>
              </div>
              <p className="text-sm text-muted-foreground">
                Minimum: 0ms (instant) • Recommended: 100-500ms
              </p>
            </div>

            {/* Quick Presets */}
            <div className="space-y-2">
              <Label className="text-sm">Quick Presets</Label>
              <div className="flex flex-wrap gap-2">
                {[0, 100, 200, 300, 500, 1000].map((preset) => (
                  <Button
                    key={preset}
                    variant={timeDelay === preset ? 'default' : 'outline'}
                    size="sm"
                    onClick={() => setTimeDelay(preset)}
                  >
                    {preset}ms
                  </Button>
                ))}
              </div>
            </div>

            {/* Visual indicator */}
            {timeDelay >= 0 && (
              <div className="p-3 rounded-lg bg-muted">
                <p className="text-sm">
                  <strong>Current setting:</strong>{' '}
                  {timeDelay === 0 
                    ? 'Instant expansion (no delay)'
                    : `Wait ${timeDelay}ms after typing stops`
                  }
                </p>
              </div>
            )}
          </div>

          {/* Action Buttons */}
          <div className="flex gap-3 pt-4">
            <Button onClick={handleSave} className="flex-1" size="lg">
              Save Settings
            </Button>
            <Button onClick={handleReset} variant="outline" size="lg">
              Reset to Default
            </Button>
          </div>
        </CardContent>
      </Card>

      {/* Additional Info Card */}
      <Card>
        <CardHeader>
          <CardTitle>Understanding Time Delay</CardTitle>
        </CardHeader>
        <CardContent className="space-y-4 text-sm">
          <div>
            <h4 className="font-semibold mb-1">⚡ 0ms - Instant</h4>
            <p className="text-muted-foreground">
              Expands immediately after you type the last character of your trigger. Very fast but may 
              occasionally trigger before you're done typing.
            </p>
          </div>
          <div>
            <h4 className="font-semibold mb-1">⏱️ 100-300ms - Quick</h4>
            <p className="text-muted-foreground">
              Good balance between speed and accuracy. Gives you a brief moment to continue typing if needed.
            </p>
          </div>
          <div>
            <h4 className="font-semibold mb-1">🐢 500-1000ms - Relaxed</h4>
            <p className="text-muted-foreground">
              More deliberate expansion. Useful if you type quickly and want to ensure you've finished the trigger.
            </p>
          </div>
          <div className="pt-2 border-t">
            <p className="text-xs text-muted-foreground">
              <strong>Tip:</strong> Try different values to find what works best for your typing speed. 
              The default 200ms works well for most users.
            </p>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}