import brandMark from '../assets/openclaw-mark.svg';

interface BrandMarkProps {
  className?: string;
  alt?: string;
}

export function BrandMark({ className, alt = 'OpenClaw Manager logo' }: BrandMarkProps) {
  return <img src={brandMark} alt={alt} className={className} draggable={false} />;
}
