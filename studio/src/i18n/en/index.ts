import common from './common';
import ui from './ui';
import shell from './shell';
import auth from './auth';
import users from './users';
import entries from './entries';
import fields from './fields';
import assets from './assets';
import setup from './setup';
import search from './search';
import trash from './trash';
import graphql from './graphql';
import ai from './ai';

// one file per area, `common` for words they share
export default {
	...common,
	...ui,
	...shell,
	...auth,
	...users,
	...entries,
	...fields,
	...assets,
	...setup,
	...search,
	...trash,
	...graphql,
	...ai,
};
