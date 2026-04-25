import { Play, Square, RotateCcw, Stethoscope } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import clsx from 'clsx';

interface ServiceStatus {
  running: boolean;
  pid: number | null;
  port: number;
}

interface QuickActionsProps {
  status: ServiceStatus | null;
  loading: boolean;
  onStart: () => void;
  onStop: () => void;
  onRestart: () => void;
}

export function QuickActions({
  status,
  loading,
  onStart,
  onStop,
  onRestart,
}: QuickActionsProps) {
  const { t } = useTranslation();
  const isRunning = status?.running || false;

  const actions = [
    {
      id: 'start',
      title: t('quickActions.start', { defaultValue: 'Start' }),
      description: 'Bring the gateway online',
      icon: Play,
      onClick: onStart,
      disabled: loading || isRunning,
      tone: 'green',
    },
    {
      id: 'stop',
      title: t('quickActions.stop', { defaultValue: 'Stop' }),
      description: 'Stop the active runtime',
      icon: Square,
      onClick: onStop,
      disabled: loading || !isRunning,
      tone: 'red',
    },
    {
      id: 'restart',
      title: t('quickActions.restart', { defaultValue: 'Restart' }),
      description: 'Recycle the process cleanly',
      icon: RotateCcw,
      onClick: onRestart,
      disabled: loading,
      tone: 'amber',
    },
    {
      id: 'diagnose',
      title: t('quickActions.diagnose', { defaultValue: 'Diagnose' }),
      description: 'Inspect logs and health checks',
      icon: Stethoscope,
      onClick: undefined,
      disabled: loading,
      tone: 'purple',
    },
  ] as const;

  return (
    <div
      className="rounded-[28px] border p-6 md:p-7"
      style={{
        background:
          'linear-gradient(180deg, color-mix(in srgb, var(--bg-card) 92%, transparent), color-mix(in srgb, var(--bg-card-hover) 96%, transparent))',
        borderColor: 'var(--border-primary)',
        boxShadow: 'var(--shadow-card)',
      }}
    >
      <div className="mb-5 flex items-end justify-between gap-4">
        <div>
          <h3 className="text-2xl font-semibold tracking-tight text-content-primary">
            {t('quickActions.title', { defaultValue: 'Quick Actions' })}
          </h3>
          <p className="mt-1 text-sm text-content-secondary">
            Core service operations, rebuilt as a proper control deck.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
        {actions.map((action) => {
          const Icon = action.icon;
          const toneClasses =
            action.tone === 'green'
              ? 'text-green-600 dark:text-green-400 bg-green-500/10'
              : action.tone === 'red'
                ? 'text-red-500 bg-red-500/10'
                : action.tone === 'amber'
                  ? 'text-amber-500 bg-amber-500/10'
                  : 'text-accent-purple bg-accent-purple/10';

          return (
            <button
              key={action.id}
              onClick={action.onClick}
              disabled={action.disabled || !action.onClick}
              className={clsx(
                'group rounded-[24px] border p-5 text-left transition-all duration-200',
                'disabled:cursor-not-allowed disabled:opacity-55',
                action.disabled || !action.onClick ? '' : 'hover:-translate-y-1'
              )}
              style={{
                backgroundColor: 'var(--bg-elevated)',
                borderColor: 'var(--border-secondary)',
                boxShadow: action.disabled || !action.onClick ? 'none' : 'var(--shadow-card)',
              }}
            >
              <div className={clsx('mb-4 flex h-12 w-12 items-center justify-center rounded-2xl', toneClasses)}>
                <Icon size={20} className={clsx(action.id === 'restart' && loading ? 'animate-spin' : '')} />
              </div>
              <p className="text-base font-semibold text-content-primary">{action.title}</p>
              <p className="mt-2 text-sm leading-relaxed text-content-secondary">{action.description}</p>
            </button>
          );
        })}
      </div>
    </div>
  );
}
