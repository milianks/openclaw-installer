import { Activity, Cpu, HardDrive, Clock, Radio } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import clsx from 'clsx';

interface ServiceStatus {
  running: boolean;
  pid: number | null;
  port: number;
  uptime_seconds: number | null;
  memory_mb: number | null;
  cpu_percent: number | null;
}

interface StatusCardProps {
  status: ServiceStatus | null;
  loading: boolean;
}

export function StatusCard({ status, loading }: StatusCardProps) {
  const { t } = useTranslation();

  const formatUptime = (seconds: number | null) => {
    if (!seconds) return '--';
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  };

  const statusLabel = loading
    ? t('statusCard.checking', { defaultValue: 'Checking' })
    : status?.running
      ? t('statusCard.running', { defaultValue: 'Running' })
      : t('statusCard.stopped', { defaultValue: 'Stopped' });

  return (
    <div
      className="relative overflow-hidden rounded-[28px] border p-6 md:p-7"
      style={{
        background:
          'linear-gradient(180deg, color-mix(in srgb, var(--bg-card) 92%, transparent), color-mix(in srgb, var(--bg-card-hover) 96%, transparent))',
        borderColor: 'var(--border-primary)',
        boxShadow: 'var(--shadow-card)',
      }}
    >
      <div className="pointer-events-none absolute inset-0 bg-gradient-radial opacity-60" />

      <div className="relative z-10 mb-6 flex flex-wrap items-start justify-between gap-4">
        <div>
          <div className="mb-3 inline-flex items-center gap-2 rounded-full border px-3 py-1 text-[11px] font-semibold uppercase tracking-[0.22em]" style={{ borderColor: 'var(--border-secondary)', color: 'var(--text-tertiary)' }}>
            <Radio size={12} />
            Runtime
          </div>
          <h3 className="text-2xl font-semibold tracking-tight text-content-primary">
            {t('statusCard.title', { defaultValue: 'Service Status' })}
          </h3>
          <p className="mt-1 text-sm text-content-secondary">
            {status?.running
              ? 'Gateway is live and listening for local traffic.'
              : 'Gateway is offline or still warming up.'}
          </p>
        </div>

        <div
          className="inline-flex items-center gap-2 rounded-full border px-4 py-2"
          style={{
            borderColor: loading
              ? 'rgba(209, 149, 57, 0.28)'
              : status?.running
                ? 'rgba(115, 154, 82, 0.28)'
                : 'rgba(201, 92, 52, 0.22)',
            backgroundColor: loading
              ? 'rgba(209, 149, 57, 0.08)'
              : status?.running
                ? 'rgba(115, 154, 82, 0.08)'
                : 'rgba(201, 92, 52, 0.08)',
          }}
        >
          <div
            className={clsx(
              'status-dot',
              loading ? 'warning' : status?.running ? 'running' : 'stopped'
            )}
          />
          <span
            className={clsx(
              'text-sm font-medium',
              loading
                ? 'text-yellow-500'
                : status?.running
                  ? 'text-green-600 dark:text-green-400'
                  : 'text-red-500'
            )}
          >
            {statusLabel}
          </span>
        </div>
      </div>

      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
        <div className="rounded-2xl border p-4" style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-secondary)' }}>
          <div className="mb-3 flex h-11 w-11 items-center justify-center rounded-2xl bg-accent-cyan/10 text-accent-cyan">
            <Activity size={18} />
          </div>
          <p className="text-xs font-medium uppercase tracking-[0.18em] text-content-tertiary">Port</p>
          <p className="mt-2 text-2xl font-semibold text-content-primary">{status?.port || 18789}</p>
        </div>

        <div className="rounded-2xl border p-4" style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-secondary)' }}>
          <div className="mb-3 flex h-11 w-11 items-center justify-center rounded-2xl bg-accent-purple/10 text-accent-purple">
            <Cpu size={18} />
          </div>
          <p className="text-xs font-medium uppercase tracking-[0.18em] text-content-tertiary">PID</p>
          <p className="mt-2 text-2xl font-semibold text-content-primary">{status?.pid || '--'}</p>
        </div>

        <div className="rounded-2xl border p-4" style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-secondary)' }}>
          <div className="mb-3 flex h-11 w-11 items-center justify-center rounded-2xl bg-accent-green/10 text-accent-green">
            <HardDrive size={18} />
          </div>
          <p className="text-xs font-medium uppercase tracking-[0.18em] text-content-tertiary">Memory</p>
          <p className="mt-2 text-2xl font-semibold text-content-primary">
            {status?.memory_mb ? `${status.memory_mb.toFixed(1)} MB` : '--'}
          </p>
        </div>

        <div className="rounded-2xl border p-4" style={{ backgroundColor: 'var(--bg-elevated)', borderColor: 'var(--border-secondary)' }}>
          <div className="mb-3 flex h-11 w-11 items-center justify-center rounded-2xl bg-accent-amber/10 text-accent-amber">
            <Clock size={18} />
          </div>
          <p className="text-xs font-medium uppercase tracking-[0.18em] text-content-tertiary">Uptime</p>
          <p className="mt-2 text-2xl font-semibold text-content-primary">
            {formatUptime(status?.uptime_seconds || null)}
          </p>
        </div>
      </div>
    </div>
  );
}
