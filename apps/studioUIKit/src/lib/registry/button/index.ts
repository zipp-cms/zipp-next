import { cva } from 'class-variance-authority';

const buttonVariants = cva(
	'px-4 py-2 rounded-full cursor-pointer active:scale-[.98] transition-all',
	{
		variants: {
			variant: {
				primary: 'bg-primary text-background hover:bg-primary/80 shadow-sm',
				default: 'bg-foreground text-background hover:bg-foreground/80 shadow-sm',
				outline: 'border border-foreground/20 text-foreground hover:bg-foreground/5',
				ghost: 'text-foreground hover:bg-foreground/5'
			}
		}
	}
);

export { buttonVariants };
