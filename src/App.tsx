import { useState, useEffect, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { Sidebar } from './components/Layout/Sidebar';
import { Header } from './components/Layout/Header';
import { Dashboard } from './components/Dashboard';
import { AIConfig } from './components/AIConfig';
import { Channels } from './components/Channels';
import { Skills } from './components/Skills';
import { Agents } from './components/Agents';
import { Settings } from './components/Settings';
import { Security } from './components/Security';
import { Testing } from './components/Testing';
import { Logs } from './components/Logs';
import { BrandMark } from './components/BrandMark';
import { appLogger } from './lib/logger';
import { isTauri } from './lib/tauri';
import { ThemeProvider } from './lib/ThemeContext';
import { Download, X, Loader2, CheckCircle, AlertCircle } from 'lucide-react';

export type PageType =
  | 'dashboard'
  | 'ai'
  | 'agents'
  | 'channels'
  | 'skills'
  | 'testing'
  | 'logs'
  | 'security'
  | 'settings';

export interface EnvironmentStatus {
  node_installed: boolean;
  node_version: string | null;
  node_version_ok: boolean;
  openclaw_installed: boolean;
  openclaw_version: string | null;
  config_dir_exists: boolean;
  ready: boolean;
  os: string;
}

interface ServiceStatus {
  running: boolean;
  pid: number | null;
  port: number;
}

interface UpdateInfo {
  update_available: boolean;
  current_version: string | null;
  latest_version: string | null;
  error: string | null;
}

interface UpdateResult {
  success: boolean;
  message: string;
  error?: string;
}

function App() {
  const { t } = useTranslation();
  const [currentPage, setCurrentPage] = useState<PageType>('dashboard');
  const [isReady, setIsReady] = useState<boolean | null>(null);
  const [envStatus, setEnvStatus] = useState<EnvironmentStatus | null>(null);
  const [serviceStatus, setServiceStatus] = useState<ServiceStatus | null>(null);
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [showUpdateBanner, setShowUpdateBanner] = useState(false);
  const [updating, setUpdating] = useState(false);
  const [updateResult, setUpdateResult] = useState<UpdateResult | null>(null);

  const checkEnvironment = useCallback(async () => {
    if (!isTauri()) {
      appLogger.warn('Not running inside Tauri, skipping environment check');
      setIsReady(true);
      return;
    }

    appLogger.info('Checking environment...');
    try {
      const status = await invoke<EnvironmentStatus>('check_environment');
      setEnvStatus(status);
      setIsReady(true);
    } catch (e) {
      appLogger.error('Environment check failed', e);
      setIsReady(true);
    }
  }, []);

  const checkUpdate = useCallback(async () => {
    if (!isTauri()) return;

    appLogger.info('Checking for OpenClaw updates...');
    try {
      const info = await invoke<UpdateInfo>('check_openclaw_update');
      setUpdateInfo(info);
      if (info.update_available) {
        setShowUpdateBanner(true);
      }
    } catch (e) {
      appLogger.error('Update check failed', e);
    }
  }, []);

  const handleUpdate = async () => {
    setUpdating(true);
    setUpdateResult(null);
    try {
      const result = await invoke<UpdateResult>('update_openclaw');
      setUpdateResult(result);
      if (result.success) {
        await checkEnvironment();
        setTimeout(() => {
          setShowUpdateBanner(false);
          setUpdateResult(null);
        }, 3000);
      }
    } catch (e) {
      setUpdateResult({
        success: false,
        message: t('app.updateError', { defaultValue: 'An error occurred during update' }),
        error: String(e),
      });
    } finally {
      setUpdating(false);
    }
  };

  useEffect(() => {
    appLogger.info('App mounted');
    checkEnvironment();
  }, [checkEnvironment]);

  useEffect(() => {
    if (!isTauri()) return;
    const timer = setTimeout(() => {
      checkUpdate();
    }, 2000);
    return () => clearTimeout(timer);
  }, [checkUpdate]);

  useEffect(() => {
    if (!isTauri()) return;

    const fetchServiceStatus = async () => {
      try {
        const status = await invoke<ServiceStatus>('get_service_status');
        setServiceStatus(status);
      } catch {
        // keep silent during polling
      }
    };

    fetchServiceStatus();
    const interval = setInterval(fetchServiceStatus, 3000);
    return () => clearInterval(interval);
  }, []);

  const handleSetupComplete = useCallback(() => {
    appLogger.info('Setup completed');
    checkEnvironment();
  }, [checkEnvironment]);

  const handleNavigate = (page: PageType) => {
    appLogger.action('Page navigation', { from: currentPage, to: page });
    setCurrentPage(page);
  };

  const renderPage = () => {
    const pageVariants = {
      initial: { opacity: 0, x: 20 },
      animate: { opacity: 1, x: 0 },
      exit: { opacity: 0, x: -20 },
    };

    const pages: Record<PageType, JSX.Element> = {
      dashboard: <Dashboard envStatus={envStatus} onSetupComplete={handleSetupComplete} />,
      ai: <AIConfig />,
      agents: <Agents />,
      channels: <Channels />,
      skills: <Skills />,
      testing: <Testing />,
      logs: <Logs />,
      security: <Security />,
      settings: <Settings onEnvironmentChange={checkEnvironment} />,
    };

    return (
      <AnimatePresence mode="wait">
        <motion.div
          key={currentPage}
          variants={pageVariants}
          initial="initial"
          animate="animate"
          exit="exit"
          transition={{ duration: 0.2 }}
          className="h-full"
        >
          {pages[currentPage]}
        </motion.div>
      </AnimatePresence>
    );
  };

  if (isReady === null) {
    return (
      <ThemeProvider>
        <div className="relative flex h-screen items-center justify-center overflow-hidden" style={{ backgroundColor: 'var(--bg-app)' }}>
          <div className="fixed inset-0 bg-gradient-radial pointer-events-none" />
          <div className="relative z-10 text-center glass-card rounded-[32px] px-10 py-9">
            <div className="mb-5 inline-flex h-20 w-20 items-center justify-center rounded-[24px] bg-gradient-to-br from-claw-400 via-claw-500 to-claw-700 animate-pulse shadow-glow-claw p-3">
              <BrandMark className="h-full w-full" />
            </div>
            <p className="text-[11px] font-semibold uppercase tracking-[0.32em]" style={{ color: 'var(--text-tertiary)' }}>
              OpenClaw
            </p>
            <h2 className="mt-2 text-2xl font-semibold" style={{ color: 'var(--text-primary)' }}>
              Manager
            </h2>
            <p className="mt-2 text-sm" style={{ color: 'var(--text-secondary)' }}>
              {t('app.starting', { defaultValue: 'Starting...' })}
            </p>
          </div>
        </div>
      </ThemeProvider>
    );
  }

  return (
    <ThemeProvider>
      <div className="relative h-screen overflow-hidden px-4 py-4 md:px-5 md:py-5" style={{ backgroundColor: 'var(--bg-app)' }}>
        <div className="fixed inset-0 bg-gradient-radial pointer-events-none" />
        <div
          className="pointer-events-none fixed inset-0 opacity-60"
          style={{
            backgroundImage:
              'linear-gradient(rgba(255,255,255,0.04) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.04) 1px, transparent 1px)',
            backgroundSize: '40px 40px',
          }}
        />

        <AnimatePresence>
          {showUpdateBanner && updateInfo?.update_available && (
            <motion.div
              initial={{ opacity: 0, y: -50 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -50 }}
              className="fixed top-4 left-1/2 z-50 w-[min(960px,calc(100vw-2rem))] -translate-x-1/2 overflow-hidden rounded-2xl border shadow-2xl backdrop-blur-xl titlebar-no-drag pointer-events-auto"
              style={{
                background: 'linear-gradient(135deg, rgba(111, 125, 217, 0.92), rgba(201, 92, 52, 0.92))',
                borderColor: 'rgba(255,255,255,0.18)',
              }}
            >
              <button
                onClick={() => {
                  setShowUpdateBanner(false);
                  setUpdateResult(null);
                }}
                aria-label="Close update banner"
                className="titlebar-no-drag pointer-events-auto absolute top-3 right-3 z-10 inline-flex h-10 w-10 items-center justify-center rounded-xl border border-white/18 bg-white/12 text-white transition-colors hover:bg-white/20"
              >
                <X size={18} />
              </button>

              <div className="titlebar-no-drag pointer-events-auto px-5 py-4 pr-16">
                <div className="min-w-0 flex items-start gap-3">
                  {updateResult?.success ? (
                    <CheckCircle size={20} className="text-green-200" />
                  ) : updateResult && !updateResult.success ? (
                    <AlertCircle size={20} className="text-red-200" />
                  ) : (
                    <Download size={20} className="text-white" />
                  )}
                  <div className="min-w-0 flex-1">
                    {updateResult ? (
                      <p className={`text-sm font-medium ${updateResult.success ? 'text-green-50' : 'text-red-50'}`}>
                        {updateResult.message}
                      </p>
                    ) : (
                      <>
                        <p className="truncate text-sm font-medium text-white">
                          {t('app.newVersion', {
                            version: updateInfo.latest_version,
                            defaultValue: `New version available: OpenClaw ${updateInfo.latest_version}`,
                          })}
                        </p>
                        <p className="truncate text-xs text-white/70">
                          {t('app.currentVersion', {
                            version: updateInfo.current_version,
                            defaultValue: `Current version: ${updateInfo.current_version}`,
                          })}
                        </p>
                      </>
                    )}
                  </div>
                </div>

                {!updateResult && (
                  <div className="mt-3">
                    <button
                      onClick={handleUpdate}
                      disabled={updating}
                      className="titlebar-no-drag pointer-events-auto inline-flex items-center gap-2 rounded-xl border border-white/20 bg-white/16 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-white/22 disabled:opacity-50"
                    >
                      {updating ? (
                        <>
                          <Loader2 size={14} className="animate-spin" />
                          {t('app.updating', { defaultValue: 'Updating...' })}
                        </>
                      ) : (
                        <>
                          <Download size={14} />
                          {t('app.updateNow', { defaultValue: 'Update Now' })}
                        </>
                      )}
                    </button>
                  </div>
                )}
              </div>
            </motion.div>
          )}
        </AnimatePresence>

        <div className="relative z-10 flex h-full gap-4">
          <Sidebar currentPage={currentPage} onNavigate={handleNavigate} serviceStatus={serviceStatus} />

          <div
            className="flex min-w-0 flex-1 flex-col overflow-hidden rounded-[30px] border"
            style={{
              backgroundColor: 'var(--bg-overlay)',
              borderColor: 'var(--border-primary)',
              boxShadow: 'var(--shadow-card)',
              backdropFilter: 'blur(18px)',
            }}
          >
            <Header currentPage={currentPage} />

            <main className="flex-1 overflow-hidden p-5 md:p-6 lg:p-7">
              {renderPage()}
            </main>
          </div>
        </div>
      </div>
    </ThemeProvider>
  );
}

export default App;
